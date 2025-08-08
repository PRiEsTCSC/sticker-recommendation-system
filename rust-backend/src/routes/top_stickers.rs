use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::structs::database_structs::{DatabaseConnection, TopStickerRequest};
use crate::middleware::auth::AuthData;
use uuid::Uuid;


#[post("/top-stickers")]
pub async fn get_top_stickers(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    payload: web::Json<TopStickerRequest>,
) -> impl Responder {
    let extensions = req.extensions();
    let auth_data = match extensions.get::<AuthData>() {
        Some(data) => data.clone(),
        None => {
            log::error!("Auth data not found in request extensions");
            return HttpResponse::Unauthorized().body("Auth data not found");
        }
    };

    let user_id = match Uuid::parse_str(&auth_data.id) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Invalid user ID format: {}", e);
            return HttpResponse::BadRequest().body("Invalid user ID format");
        }
    };


    match db.get_top_stickers(user_id).await {
        Ok(top_stickers) => HttpResponse::Ok().json(top_stickers),
        Err(e) => {
            log::error!("Failed to fetch top stickers for user {} (ID: {}): {}", payload.username, user_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch top stickers")
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_top_stickers);
}