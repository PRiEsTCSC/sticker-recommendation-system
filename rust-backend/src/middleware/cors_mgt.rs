use actix_cors::Cors;
use actix_web::http::header;
use std::env;

pub fn handle_cors() -> Cors {
    if let Ok(_allowed_origin) = env::var("FRONTEND_URL") {
        Cors::default()
            //            .allowed_origin(&allowed_origin) // ✅ only this is valid
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials()
    } else {
        // fallback: allow all
        eprintln!("⚠️  FRONTEND_URL not set — defaulting to permissive CORS");
        Cors::permissive()
    }
}
