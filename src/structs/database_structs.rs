use sqlx::{Pool, Postgres, FromRow};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<Postgres>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Admin {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub last_login: Option<NaiveDateTime>,
    pub failed_attempts: i32,
}


#[derive(Debug, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Option<Uuid>,  // For regular users
    pub admin_id: Option<Uuid>, // For admins
    pub token: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Interaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub input_text: String,
    pub detected_emotion: String,
    pub sticker_url: String,
    pub created_at: NaiveDateTime,
}



#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RecommendRequest {
    pub text: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct StickerMetric {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sticker_url: String,
    pub usage_count: i32,
    pub last_used: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct FavoriteRequest {
    pub sticker_url: String,
}

#[derive(Deserialize)]
pub struct UpdateUsernameRequest {
    pub new_username: String,
}

#[derive(Deserialize)]
pub struct ManagementRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}