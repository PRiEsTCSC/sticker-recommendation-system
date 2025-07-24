use std::sync::Arc;
use redis::Client;
use sqlx::{Pool, Postgres, FromRow};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;


#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<Postgres>,
    pub redis: Arc<Client>,
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

#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RecommendRequest {
    pub username: String,
    pub input_text: String,
}

#[derive(Serialize, Clone)]
pub struct RecommendResponse {
    pub detected_emotion: String,
    pub sticker_urls: Vec<String>, // Changed from sticker_url: String
}

// #[derive(Clone)]
// pub struct JwtMiddlewareStruct{
    
//     pub user : String,
//     pub credentials: BearerAuth,
// }

#[derive(Debug, FromRow, Serialize)]
pub struct StickerMetric {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sticker_url: String,
    pub usage_count: i32,
    pub last_used: NaiveDateTime,
}

// #[derive(Deserialize)]
// pub struct FavoriteRequest {
//     pub sticker_url: String,
// }

#[derive(Deserialize)]
pub struct UpdateUsernameRequest {
    pub new_username: String,
}

#[derive(Deserialize)]
pub struct ManagementRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}