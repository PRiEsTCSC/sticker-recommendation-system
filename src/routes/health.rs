use actix_web::{get, web, HttpResponse, Responder};

#[get("/health")]
async fn server_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(server_check);
}
