use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use crate::structs::database_structs::{RecommendRequest, RecommendResponse, TrendingRequest, TrendingResponse};
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

    // Log the request
    log::info!("User {} (ID: {}) requested sticker for text: {}", username, user_id, input_text);
    let client = Client::new();

    // Call Emotion Detection Service
    let emotion_response = match client
        .post("http:///sticker-api:8000/detect_emotion")
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

    // Check Redis cache for stickers
    let cache_key = format!("sticker:{}", detected_emotion);
    if let Ok(Some((emotion, cached_value))) = db.get_cached_sticker(&cache_key).await {
        if let Ok(sticker_urls) = serde_json::from_str::<Vec<String>>(&cached_value) {
            // Cache hit with a list of stickers
            if let Some(first_sticker) = sticker_urls.first() {
                save_interaction(&db, user_id, &input_text, &emotion, first_sticker).await;
            }
            return HttpResponse::Ok().json(RecommendResponse {
                detected_emotion: emotion,
                sticker_urls,
            });
        } else {
            // Cache hit with a single sticker (backward compatibility)
            save_interaction(&db, user_id, &input_text, &emotion, &cached_value).await;
            return HttpResponse::Ok().json(RecommendResponse {
                detected_emotion: emotion,
                sticker_urls: vec![cached_value],
            });
        }
    }

    // Call Sticker Search Service
    let sticker_response = match client
        .post("http:///sticker-api:8000/search_stickers")
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

    let sticker_urls: Vec<String> = sticker_results
        .iter()
        .filter_map(|item| item["url"].as_str().map(|s| s.to_string()))
        .collect();

    if !sticker_urls.is_empty() {
        // Cache all sticker URLs as a JSON string
        let sticker_urls_json = serde_json::to_string(&sticker_urls).unwrap();
        let db_clone = db.clone();
        let cache_key_clone = cache_key.clone();
        actix_web::rt::spawn(async move {
            if let Err(e) = db_clone.cache_sticker(&cache_key_clone, &sticker_urls_json).await {
                log::error!("Failed to cache stickers: {}", e);
            }
        });

        // Save interaction for the first sticker
        if let Some(first_sticker) = sticker_urls.first() {
            save_interaction(&db, user_id, &input_text, &detected_emotion, first_sticker).await;
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

















#[post("/dashboard-find")]
async fn find_sticker_dashboard(
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

    // Log the request
    log::info!("User {} (ID: {}) requested sticker for text: {}", username, user_id, input_text);
    let client = Client::new();

    // Call Emotion Detection Service
    let emotion_response = match client
        .post("http:///sticker-api:8000/detect_emotion")
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

    // Check Redis cache for stickers
    let cache_key = format!("sticker:{}", detected_emotion);
    if let Ok(Some((emotion, cached_value))) = db.get_cached_sticker(&cache_key).await {
        if let Ok(sticker_urls) = serde_json::from_str::<Vec<String>>(&cached_value) {
            // Cache hit with a list of stickers
            if let Some(first_sticker) = sticker_urls.first() {
                save_interaction(&db, user_id, &input_text, &emotion, first_sticker).await;
            }
            return HttpResponse::Ok().json(RecommendResponse {
                detected_emotion: emotion,
                sticker_urls,
            });
        } else {
            // Cache hit with a single sticker (backward compatibility)
            save_interaction(&db, user_id, &input_text, &emotion, &cached_value).await;
            return HttpResponse::Ok().json(RecommendResponse {
                detected_emotion: emotion,
                sticker_urls: vec![cached_value],
            });
        }
    }

    // Call Sticker Search Service
    let sticker_response = match client
        .post("http:///sticker-api:8000/search_stickers_dashboard")
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

    let sticker_urls: Vec<String> = sticker_results
        .iter()
        .filter_map(|item| item["url"].as_str().map(|s| s.to_string()))
        .collect();

    if !sticker_urls.is_empty() {
        // Cache all sticker URLs as a JSON string
        let sticker_urls_json = serde_json::to_string(&sticker_urls).unwrap();
        let db_clone = db.clone();
        let cache_key_clone = cache_key.clone();
        actix_web::rt::spawn(async move {
            if let Err(e) = db_clone.cache_sticker(&cache_key_clone, &sticker_urls_json).await {
                log::error!("Failed to cache stickers: {}", e);
            }
        });

        // Save interaction for the first sticker
        if let Some(first_sticker) = sticker_urls.first() {
            save_interaction(&db, user_id, &input_text, &detected_emotion, first_sticker).await;
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







#[post("/dashboard-trending")]
async fn trending_dashboard(
    req: web::Json<TrendingRequest>,
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
    log::info!("User {} (ID: {}) requested trending stickers", username, user_id);

    // Get GIPHY API key from environment variables
    let giphy_api_key = match std::env::var("GIPHY_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            log::error!("GIPHY_API_KEY is not set");
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "GIPHY_API_KEY is not set"
            }));
        }
    };

    // Fetch trending stickers from Giphy API
    let client = reqwest::Client::new();
    let params = [
        ("api_key", &giphy_api_key),
        ("limit", &"9".to_string()),
        ("rating", &"g".to_string()),
        ("bundle", &"messaging_non_clips".to_string()),
    ];
    let url = "https://api.giphy.com/v1/stickers/trending";

    match client.get(url).query(&params).send().await {
        Ok(res) => {
            if res.status().is_success() {
                let data: Value = match res.json().await {
                    Ok(data) => data,
                    Err(e) => {
                        log::error!("Failed to parse Giphy response: {}", e);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Failed to parse Giphy response"
                        }));
                    }
                };
                let sticker_urls: Vec<String> = data["data"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|item| item["images"]["original"]["url"].as_str().map(|s| s.to_string()))
                    .collect();
                HttpResponse::Ok().json(TrendingResponse {
                    sticker_urls,
                })
            } else {
                log::error!("Giphy API error: {}", res.status());
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch stickers from Giphy"
                }))
            }
        }
        Err(e) => {
            log::error!("Failed to fetch stickers: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch stickers"
            }))
        }
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
    cfg.service(find_sticker_dashboard);
    cfg.service(trending_dashboard);
    
    
}