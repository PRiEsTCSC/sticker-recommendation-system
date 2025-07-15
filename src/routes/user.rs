use actix_web::{put, delete, web, HttpResponse};
use crate::structs::database_structs::{DatabaseConnection, ManagementRequest, UpdateUsernameRequest};
use uuid::Uuid;
use actix_web::error::Error as ActixError;

#[put("/user/update-username")]
async fn update_username(
    db: web::Data<DatabaseConnection>,
    user_id: web::ReqData<String>,
    req: web::Json<UpdateUsernameRequest>,
) -> Result<HttpResponse, ActixError>{
    let user_id = Uuid::parse_str(&user_id).map_err(|_| {
        log::error!("Invalid user ID in token");
        actix_web::error::ErrorBadRequest("Invalid user ID")
    })?;
    match db.update_user(user_id, ManagementRequest {
        username: Some(req.new_username.clone()),
        password: None,
    }).await {
        Ok(user) => Ok(HttpResponse::Ok().json(serde_json::json!({ "username": user.username }))),
        Err(e) => {
            log::error!("Failed to update username for user {}: {}", user_id, e);
            Err(actix_web::error::ErrorBadRequest("Username already exists or invalid"))
        }
    }
}

#[delete("/user/delete")]
async fn delete_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, ActixError>{
    let user_id = Uuid::parse_str(&user_id).map_err(|_| {
        log::error!("Invalid user ID in token");
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
    cfg.service(update_username);
    cfg.service(delete_user);
}