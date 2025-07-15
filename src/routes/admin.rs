use actix_web::{delete, get, post, put, web, HttpResponse,};
use crate::structs::database_structs::{DatabaseConnection, RegisterRequest, ManagementRequest};
use uuid::Uuid;
use actix_web::error::Error as ActixError;

#[get("/admin/users")]
async fn list_users(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, ActixError> {
    db.get_all_users().await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| {
            log::error!("Failed to fetch users: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to fetch users")
        })
}

#[post("/admin/users")]
async fn add_user(
    db: web::Data<DatabaseConnection>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ActixError> {
    db.register_user(req.into_inner()).await
        .map(|user| HttpResponse::Ok().json(serde_json::json!({ "username": user.username })))
        .map_err(|e| {
            log::error!("Failed to add user: {}", e);
            actix_web::error::ErrorBadRequest("Username already exists")
        })
}

#[put("/admin/users/{id}")]
async fn update_user(
    db: web::Data<DatabaseConnection>,
    path: web::Path<String>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ActixError> {
    let user_id = Uuid::parse_str(&path.into_inner()).map_err(|_| {
        log::error!("Invalid user ID");
        actix_web::error::ErrorBadRequest("Invalid user ID")
    })?;
    db.update_user(user_id, ManagementRequest {
        username: Some(req.username.clone()),
        password: Some(req.password.clone()),
    }).await
        .map(|user| HttpResponse::Ok().json(serde_json::json!({ "username": user.username })))
        .map_err(|e| {
            log::error!("Failed to update user {}: {}", user_id, e);
            actix_web::error::ErrorBadRequest("Username already exists or invalid")
        })
}

#[delete("/admin/users/{id}")]
async fn delete_user(
    db: web::Data<DatabaseConnection>,
    path: web::Path<String>,
) -> Result<HttpResponse, ActixError> {
    let user_id = Uuid::parse_str(&path.into_inner()).map_err(|_| {
        log::error!("Invalid user ID");
        actix_web::error::ErrorBadRequest("Invalid user ID")
    })?;
    db.delete_user(user_id).await
        .map(|_| HttpResponse::Ok().body("User deleted"))
        .map_err(|e| {
            log::error!("Failed to delete user {}: {}", user_id, e);
            actix_web::error::ErrorInternalServerError("Failed to delete user")
        })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(add_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}