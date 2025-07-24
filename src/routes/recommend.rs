use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use crate::structs::database_structs::{RecommendRequest, RecommendResponse};
use crate::middleware::auth::AuthData;
use serde_json::Value;
use reqwest::Client;


#[post("/find")]
async fn find_sticker(
    db: web::Data<crate::structs::database_structs::DatabaseConnection>,
    req: web::Json<RecommendRequest>,
    http_req: HttpRequest,
) -> impl Responder {
    // Extract the request payload
    let req = req.into_inner();
    println!("▶️ find payload: {:?}", req);
    let input_text = req.input_text.trim().to_lowercase();
    let username = &req.username;

    // Extract AuthData from extensions
    let extensions = http_req.extensions();
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

    // Verify that the username in the request matches the authenticated user
    // You might want to add this check based on your business logic
    log::info!("User {} (ID: {}) requested sticker for text: {}", username, user_id, input_text);
	let client = Client::new();
    // Call Emotion Detection Service
    let emotion_response = match client
        .post("http://localhost:8000/detect_emotion")
        .json(&serde_json::json!({ "input_text": &input_text }))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            log::error!("Emotion detection request failed: {}", e);
            return HttpResponse::InternalServerError().body("Emotion detection failed");
        }
    };

    let detected_emotion = match emotion_response.status() {
        reqwest::StatusCode::OK => {
            match emotion_response.json::<Value>().await {
                Ok(json) => json["detected_emotion"].as_str().unwrap_or("neutral").to_string(),
                Err(e) => {
                    log::error!("Failed to parse emotion response: {}", e);
                    return HttpResponse::InternalServerError().body("Emotion detection failed");
                }
            }
        }
        _ => {
            log::error!("Emotion service returned error: {}", emotion_response.status());
            return HttpResponse::InternalServerError().body("Emotion service unavailable");
        }
    };

    // Check Redis cache for sticker
    
	let cache_key = format!("sticker:{}", detected_emotion);
	if let Ok(Some((emotion, sticker_url))) = db.get_cached_sticker(&cache_key).await {
		save_interaction(&db, user_id, &input_text, &emotion, &sticker_url).await;
		return HttpResponse::Ok().json(RecommendResponse {
		    detected_emotion: emotion,
		    sticker_urls: vec![sticker_url], // Updated to use sticker_urls
		});
	}
    // Call Sticker Search Service
    let sticker_response = match client
        .post("http://localhost:8000/search_stickers")
        .json(&serde_json::json!({ "q": &detected_emotion, "rating": "g" }))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            log::error!("Sticker search request failed: {}", e);
            return HttpResponse::InternalServerError().body("Sticker search failed");
        }
    };

    let sticker_results = match sticker_response.status() {
        reqwest::StatusCode::OK => {
            match sticker_response.json::<Vec<Value>>().await {
                Ok(results) => results,
                Err(e) => {
                    log::error!("Failed to parse sticker response: {}", e);
                    return HttpResponse::InternalServerError().body("Sticker search failed");
                }
            }
        }
        _ => {
            log::error!("Sticker service returned error: {}", sticker_response.status());
            return HttpResponse::InternalServerError().body("Sticker service unavailable");
        }
    };

    if !sticker_results.is_empty() {
        let sticker_urls: Vec<String> = sticker_results
            .iter()
            .filter_map(|item| item["url"].as_str().map(|s| s.to_string()))
            .collect();

        // Asynchronously cache the first sticker
        if let Some(sticker_url) = sticker_urls.first() {
            let db_clone = db.clone();
            let _cache_key_clone = cache_key.clone(); // Clone cache_key for the closure
            let _detected_emotion_clone = detected_emotion.clone();
            let sticker_url_clone = sticker_url.clone();
            actix_web::rt::spawn(async move {
                if let Err(e) = db_clone.cache_sticker(&cache_key, &sticker_url_clone).await {
                    log::error!("Failed to cache sticker: {}", e);
                }
            });

            // Save interaction for the first sticker
            save_interaction(&db, user_id, &input_text, &detected_emotion, sticker_url).await;
        }

        // Return all sticker URLs
        HttpResponse::Ok().json(RecommendResponse {
            detected_emotion,
            sticker_urls,
        })
    } else {
        HttpResponse::BadGateway().body("No stickers returned by service")
    }
}


async fn save_interaction(db: &web::Data<crate::structs::database_structs::DatabaseConnection>, user_id: Uuid, input_text: &str, emotion: &str, sticker_url: &str) {
    if let Err(e) = db.save_interaction(user_id, input_text, emotion, sticker_url).await {
        log::error!("Failed to save interaction: {}", e);
    }
    if let Err(e) = db.update_sticker_metrics(user_id, sticker_url).await {
        log::error!("Failed to update sticker metrics: {}", e);
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_sticker);
}