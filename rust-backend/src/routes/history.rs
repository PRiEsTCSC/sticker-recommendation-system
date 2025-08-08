use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::structs::database_structs::{DatabaseConnection, HistoryResponse, HistoryRequest};
use crate::middleware::auth::AuthData;
use uuid::Uuid;
use log;

#[post("/history")]
async fn get_history(
    db: web::Data<DatabaseConnection>,
    req: web::Json<HistoryRequest>,
    http_req: HttpRequest,
) -> impl Responder {


    // Extract the request payload
    let req = req.into_inner();
    println!("▶️ find payload: {:?}", req);
    let username = &req.username;
    println!("▶️ username payload: {:?}", username);
    // Extract AuthData from extensions
    let extensions = http_req.extensions();
    println!("▶️ extensions payload: {:?}", extensions);
    let auth_data = match extensions.get::<AuthData>() {
        Some(data) => data.clone(),
        None => {
            log::error!("Auth data not found in request extensions");
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Auth data not found"
            }));
        }
    };
    let user_id = match Uuid::parse_str(&auth_data.id) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Invalid user ID format: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user ID format"
            }));
        }
    };

    // Log the request
    log::info!("User {} (ID: {}) requested history", username, user_id);

    // Fetch user history from the database
    match db.get_user_history(user_id).await {
        Ok(history) => {
            if history.is_empty() {
                log::info!("No history found for user {} (ID: {})", username, user_id);
                return HttpResponse::Ok().json(HistoryResponse { history });
            }
            log::info!("Fetched history for user {} (ID: {}) with {} entries", username, user_id, history.len());
            HttpResponse::Ok().json(HistoryResponse { history })
        }
        Err(e) => {
            log::error!("Failed to fetch history for user {} (ID: {}): {}", username, user_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch history")
        }
    }
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_history);
}