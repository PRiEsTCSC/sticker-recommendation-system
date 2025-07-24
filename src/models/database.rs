use std::sync::Arc;

use chrono::NaiveDateTime;
use redis::{Client, Commands};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::{configs::env_load::{load_database_url, load_redis_url}, structs::database_structs::{Admin, DatabaseConnection, ManagementRequest, RegisterRequest, Session, User}};



impl DatabaseConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = load_database_url();
        let pool = Pool::<Postgres>::connect(&database_url).await?;
        let redis = Arc::new(Client::open(load_redis_url()).expect("Failed to create Redis client"));
        Ok(Self { pool, redis })
    }

    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        // Split into separate queries to avoid multiple commands in a prepared statement
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY,
                username VARCHAR NOT NULL UNIQUE,
                password_hash VARCHAR NOT NULL
            )"#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS admins (
                id UUID PRIMARY KEY,
                username VARCHAR NOT NULL UNIQUE,
                password_hash VARCHAR NOT NULL,
                last_login TIMESTAMP,
                failed_attempts INTEGER DEFAULT 0
            )"#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id UUID PRIMARY KEY,
                user_id UUID REFERENCES users(id),
                admin_id UUID REFERENCES admins(id),
                token TEXT NOT NULL,
                expires_at TIMESTAMP NOT NULL,
                CHECK (user_id IS NOT NULL OR admin_id IS NOT NULL)
            )"#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS interactions (
                id UUID PRIMARY KEY,
                user_id UUID REFERENCES users(id),
                input_text TEXT NOT NULL,
                detected_emotion VARCHAR NOT NULL,
                sticker_url TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL
            )"#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sticker_metrics (
                id UUID PRIMARY KEY,
                user_id UUID REFERENCES users(id),
                sticker_url TEXT NOT NULL,
                usage_count INTEGER NOT NULL DEFAULT 1,
                last_used TIMESTAMP NOT NULL,
                UNIQUE (user_id, sticker_url)
            )"#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }


    pub async fn save_interaction(
        &self,
        user_id: Uuid,
        input_text: &str,
        detected_emotion: &str,
        sticker_url: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO interactions (id, user_id, input_text, detected_emotion, sticker_url, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(input_text)
        .bind(detected_emotion)
        .bind(sticker_url)
        .bind(chrono::Utc::now().naive_utc())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_sticker_metrics(&self, user_id: Uuid, sticker_url: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO sticker_metrics (id, user_id, sticker_url, usage_count, last_used)
            VALUES ($1, $2, $3, 1, $4)
            ON CONFLICT (user_id, sticker_url)
            DO UPDATE SET usage_count = sticker_metrics.usage_count + 1, last_used = $4
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(sticker_url)
        .bind(chrono::Utc::now().naive_utc())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // pub async fn get_sticker_metrics(&self, user_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
    //     let stickers = sqlx::query_scalar::<_, String>("SELECT sticker_url FROM sticker_metrics WHERE user_id = $1")
    //         .bind(user_id)
    //         .fetch_all(&self.pool)
    //         .await?;
    //     Ok(stickers)
    // }

    pub async fn cache_sticker(&self, emotion: &str, sticker_url: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.get_connection()?;
        let value = serde_json::json!({
            "detected_emotion": emotion,
            "sticker_url": sticker_url
        }).to_string();
        conn.set_ex::<_, _, ()>(format!("sticker:{}", emotion), value, 3600)?; // Cache for 1 hour
        Ok(())
    }

    pub async fn get_cached_sticker(&self, emotion: &str) -> Result<Option<(String, String)>, redis::RedisError> {
        let mut conn = self.redis.get_connection()?;
        let key = format!("sticker:{}", emotion);
        if let Some(value) = conn.get::<_, Option<String>>(&key)? {
            let json: serde_json::Value = serde_json::from_str(&value).expect("Error on database.rs");
            let emotion = json["detected_emotion"].as_str().unwrap_or("neutral").to_string();
            let sticker_url = json["sticker_url"].as_str().unwrap_or_default().to_string();
            Ok(Some((emotion, sticker_url)))
        } else {
            Ok(None)
        }
    }

    pub async fn save_session(
        &self,
        user_id: Option<Uuid>,
        admin_id: Option<Uuid>,
        token: &str,
        expires_at: NaiveDateTime,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sessions (id, user_id, admin_id, token, expires_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(admin_id)
        .bind(token)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE token = $1 AND expires_at > NOW()",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        Ok(session)
    }

    // pub async fn delete_session(&self, token: &str) -> Result<(), sqlx::Error> {
    //     sqlx::query("DELETE FROM sessions WHERE token = $1")
    //         .bind(token)
    //         .execute(&self.pool)
    //         .await?;
    //     Ok(())
    // }

    pub async fn update_admin_login(&self, admin_id: Uuid, success: bool) -> Result<(), sqlx::Error> {
        if success {
            sqlx::query("UPDATE admins SET last_login = NOW(), failed_attempts = 0 WHERE id = $1")
                .bind(admin_id)
                .execute(&self.pool)
                .await?;
        } else {
            sqlx::query("UPDATE admins SET failed_attempts = failed_attempts + 1 WHERE id = $1")
                .bind(admin_id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }












    ////////////////////////////////////////////  USER MANAGEMENT FUNCTIONS ////////////////////////////////////////////
    pub async fn register_user(&self, req: RegisterRequest) -> Result<User, sqlx::Error> {
        let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");
        let user_id = Uuid::new_v4();
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(&req.username)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        req: ManagementRequest,
    ) -> Result<User, sqlx::Error> {
        let mut query_parts = vec![];
        let mut param_idx = 1;

        // Store optional concrete values directly
        let mut username_val: Option<String> = None;
        let mut password_hash_val: Option<String> = None;

        if let Some(username) = req.username {
            query_parts.push(format!("username = ${}", param_idx));
            username_val = Some(username);
            param_idx += 1;
        }
        if let Some(password) = req.password {
            let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
                .expect("Failed to hash password for update");
            query_parts.push(format!("password_hash = ${}", param_idx));
            password_hash_val = Some(password_hash);
            param_idx += 1;
        }

        if query_parts.is_empty() {
            log::warn!("Attempted to update user {} with no provided fields (username or password).", user_id);
            return sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await;
        }

        let query_string = format!(
            "UPDATE users SET {} WHERE id = ${} RETURNING *",
            query_parts.join(", "),
            param_idx // This param_idx is for the user_id in the WHERE clause
        );

        // Start building the query and chain binds
        let mut sqlx_query = sqlx::query_as::<_, User>(&query_string);

        // Conditionally bind the values based on their presence
        if let Some(val) = username_val {
            sqlx_query = sqlx_query.bind(val);
        }
        if let Some(val) = password_hash_val {
            sqlx_query = sqlx_query.bind(val);
        }
        // Finally, bind the user_id for the WHERE clause
        sqlx_query = sqlx_query.bind(user_id);


        let user = sqlx_query.fetch_one(&self.pool).await?;
        Ok(user)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
         let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
             .bind(user_id)
             .fetch_optional(&self.pool)
            .await?;
         Ok(user)
    }
////////////////////////////////////////////////  ////////////////////////////////////////////
    





    ////////////////////////////////////////////  ADMIN MANAGEMENT FUNCTIONS ////////////////////////////////////////////
    pub async fn register_admin(&self, req: RegisterRequest) -> Result<Admin, sqlx::Error> {
        let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");
        let admin_id = Uuid::new_v4();
        let admin = sqlx::query_as::<_, Admin>(
            "INSERT INTO admins (id, username, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(admin_id)
        .bind(&req.username)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(admin)
    }

    // pub async fn update_admin(&self, admin_id: Uuid, req: ManagementRequest) -> Result<Admin, sqlx::Error> {
    //     let mut query_parts = vec![];
    //     let mut param_idx = 1;

    //     // Store optional concrete values directly for admin
    //     let mut username_val: Option<String> = None;
    //     let mut password_hash_val: Option<String> = None;

    //     if let Some(username) = req.username {
    //         query_parts.push(format!("username = ${}", param_idx));
    //         username_val = Some(username);
    //         param_idx += 1;
    //     }
    //     if let Some(password) = req.password {
    //         let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST).expect("Failed to hash password");
    //         query_parts.push(format!("password_hash = ${}", param_idx));
    //         password_hash_val = Some(password_hash);
    //         param_idx += 1;
    //     }

    //     if query_parts.is_empty() {
    //         log::warn!("Attempted to update admin {} with no provided fields (username or password).", admin_id);
    //         return sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE id = $1")
    //             .bind(admin_id)
    //             .fetch_one(&self.pool)
    //             .await;
    //     }

    //     let query_string = format!(
    //         "UPDATE admins SET {} WHERE id = ${} RETURNING *",
    //         query_parts.join(", "),
    //         param_idx // This param_idx is for the admin_id in the WHERE clause
    //     );

    //     // Start building the query and chain binds for admin
    //     let mut sqlx_query = sqlx::query_as::<_, Admin>(&query_string);

    //     // Conditionally bind the values
    //     if let Some(val) = username_val {
    //         sqlx_query = sqlx_query.bind(val);
    //     }
    //     if let Some(val) = password_hash_val {
    //         sqlx_query = sqlx_query.bind(val);
    //     }
    //     // Finally, bind the admin_id for the WHERE clause
    //     sqlx_query = sqlx_query.bind(admin_id);

    //     let admin = sqlx_query.fetch_one(&self.pool).await?;
    //     Ok(admin)
    // }

    // pub async fn delete_admin(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
    //     sqlx::query("DELETE FROM admins WHERE id = $1")
    //         .bind(user_id)
    //         .execute(&self.pool)
    //         .await?;
    //     Ok(())
    // }

    pub async fn get_admin_by_username(&self, username: &str) -> Result<Option<Admin>, sqlx::Error> {
        let admin = sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(admin)
    }

    // pub async fn get_admin_by_id(&self, admin_id: Uuid) -> Result<Option<Admin>, sqlx::Error> {
    //     let admin = sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE id = $1")
    //         .bind(admin_id)
    //         .fetch_optional(&self.pool)
    //         .await?;
    //     Ok(admin)
    // }
        //////////////////////////////////////////////  ////////////////////////////////////////////


}