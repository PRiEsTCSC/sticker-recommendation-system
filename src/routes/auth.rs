use crate::middleware::auth::{create_token, AuthConfig};
use crate::structs::database_structs::{DatabaseConnection, LoginRequest, RegisterRequest};
use actix_web::error::Error as ActixError;
use actix_web::{post, web, HttpResponse};
use bcrypt::verify;
use chrono::{Duration, Utc};
use serde_json::json;

#[post("/auth/register/user")]
async fn register_user(
    db: web::Data<DatabaseConnection>,
    auth_config: web::Data<AuthConfig>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ActixError> {
    println!("▶️ Register payload: {:?}", req);

    let user = db.register_user(req.into_inner()).await.map_err(|e| {
        log::warn!("User registration failed: {}", e);
        actix_web::error::InternalError::new(
            json!({ "error": "Username already exists" }).to_string(),
            actix_web::http::StatusCode::BAD_REQUEST,
        )
    })?;

    let token = create_token(&user.id.to_string(), "user", auth_config.get_ref()).map_err(|e| {
        log::error!("Failed to generate token: {}", e);
        actix_web::error::ErrorInternalServerError(json!({ "error": "Failed to generate token" }))
    })?;
    let expires_at = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .naive_utc();
    db.save_session(Some(user.id), None, &token, expires_at)
        .await
        .map_err(|e| {
            log::error!("Failed to save session for user {}: {}", user.id, e);
            actix_web::error::ErrorInternalServerError(json!({ "error": "Failed to save session"}))
        })?;
    Ok(HttpResponse::Ok().json(json!({ "token": token, "username": user.username })))
}

#[post("/auth/login/user")]
async fn login_user(
    db: web::Data<DatabaseConnection>,
    auth_config: web::Data<AuthConfig>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, ActixError> {
    let user = db
        .get_user_by_username(&req.username)
        .await
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError(json!({ "error": "Database error"}))
        })?
        .ok_or_else(|| {
            log::warn!("Login failed: Invalid username");
            actix_web::error::ErrorUnauthorized(json!({ "error": "Invalid credentials"}))
        })?;
    if !verify(&req.password, &user.password_hash).unwrap_or(false) {
        log::warn!("Login failed: Invalid password for {}", req.username);
        return Err(actix_web::error::ErrorUnauthorized(
            json!({ "error": "Invalid credentials"}),
        ));
    }
    let token = create_token(&user.id.to_string(), "user", auth_config.get_ref()).map_err(|e| {
        log::error!("Failed to generate token: {}", e);
        actix_web::error::ErrorInternalServerError(json!({ "error": "Failed to generate token"}))
    })?;
    let expires_at = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .naive_utc();
    db.save_session(Some(user.id), None, &token, expires_at)
        .await
        .map_err(|e| {
            log::error!("Failed to save session for user {}: {}", user.id, e);
            actix_web::error::ErrorInternalServerError(json!({ "error": "Failed to save session"}))
        })?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token, "username": user.username })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user);
}
