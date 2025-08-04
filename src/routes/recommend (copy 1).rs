// use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
// use uuid::Uuid;
// use crate::structs::database_structs::{RecommendRequest, RecommendResponse, Sticker};
// use crate::middleware::auth::AuthData;
// use serde_json::Value;
// use reqwest::Client;

// use flate2::write::ZlibEncoder;
// use flate2::read::ZlibDecoder;
// use flate2::Compression;
// use std::io::{Read, Write};
// use base64::{engine::general_purpose, Engine as _};

// #[post("/find")]
// async fn find_sticker(
//     db: web::Data<crate::structs::database_structs::DatabaseConnection>,
//     req: web::Json<RecommendRequest>,
//     http_req: HttpRequest,
// ) -> impl Responder {
//     // Extract the request payload
//     let req = req.into_inner();
//     println!("▶️ find payload: {:?}", req);
//     let input_text = req.input_text.trim().to_lowercase();
//     let username = &req.username;

//     // Extract AuthData from extensions
//     let extensions = http_req.extensions();
//     let auth_data = match extensions.get::<AuthData>() {
//         Some(data) => data.clone(),
//         None => {
//             log::error!("Auth data not found in request extensions");
//             return HttpResponse::Unauthorized().body("Auth data not found");
//         }
//     };
//     let user_id = match Uuid::parse_str(&auth_data.id) {
//         Ok(id) => id,
//         Err(e) => {
//             log::error!("Invalid user ID format: {}", e);
//             return HttpResponse::BadRequest().body("Invalid user ID format");
//         }
//     };

//     // Verify that the username in the request matches the authenticated user
//     // You might want to add this check based on your business logic
//     log::info!("User {} (ID: {}) requested sticker for text: {}", username, user_id, input_text);
// 	let client = Client::new();
//     // Call Emotion Detection Service
//     let emotion_response = match client
//         .post("http://localhost:8000/detect_emotion")
//         .json(&serde_json::json!({ "input_text": &input_text }))
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(e) => {
//             log::error!("Emotion detection request failed: {}", e);
//             return HttpResponse::InternalServerError().body("Emotion detection failed");
//         }
//     };

//     let detected_emotion = match emotion_response.status() {
//         reqwest::StatusCode::OK => {
//             match emotion_response.json::<Value>().await {
//                 Ok(json) => json["detected_emotion"].as_str().unwrap_or("neutral").to_string(),
//                 Err(e) => {
//                     log::error!("Failed to parse emotion response: {}", e);
//                     return HttpResponse::InternalServerError().body("Emotion detection failed");
//                 }
//             }
//         }
//         _ => {
//             log::error!("Emotion service returned error: {}", emotion_response.status());
//             return HttpResponse::InternalServerError().body("Emotion service unavailable");
//         }
//     };

//     // Check Redis cache for sticker

// 	let cache_key = format!("sticker:{}", detected_emotion);
// 	if let Ok(Some((emotion, sticker_url))) = db.get_cached_sticker(&cache_key).await {
// 		save_interaction(&db, user_id, &input_text, &emotion, &sticker_url).await;
// 		return HttpResponse::Ok().json(RecommendResponse {
// 		    detected_emotion: emotion,
// 		    sticker_urls: vec![sticker_url], // Updated to use sticker_urls
// 		});
// 	}
//     // Call Sticker Search Service
//     let sticker_response = match client
//         .post("http://localhost:8000/search_stickers")
//         .json(&serde_json::json!({ "q": &detected_emotion, "rating": "g" }))
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(e) => {
//             log::error!("Sticker search request failed: {}", e);
//             return HttpResponse::InternalServerError().body("Sticker search failed");
//         }
//     };

//     let sticker_results = match sticker_response.status() {
//         reqwest::StatusCode::OK => {
//             match sticker_response.json::<Vec<Value>>().await {
//                 Ok(results) => results,
//                 Err(e) => {
//                     log::error!("Failed to parse sticker response: {}", e);
//                     return HttpResponse::InternalServerError().body("Sticker search failed");
//                 }
//             }
//         }
//         _ => {
//             log::error!("Sticker service returned error: {}", sticker_response.status());
//             return HttpResponse::InternalServerError().body("Sticker service unavailable");
//         }
//     };

//     if !sticker_results.is_empty() {
//         let sticker_urls: Vec<String> = sticker_results
//             .iter()
//             .filter_map(|item| item["url"].as_str().map(|s| s.to_string()))
//             .collect();

//         // Asynchronously cache all stickers (up to 3)
//         let stickers_to_cache = sticker_urls.iter().take(3).cloned().collect::<Vec<String>>();
//         for sticker_url in &stickers_to_cache {
//             let db_clone = db.clone();
//             let cache_key = cache_key.clone();
//             let sticker_url_clone = sticker_url.clone();
//             actix_web::rt::spawn(async move {
//             if let Err(e) = db_clone.cache_sticker(&cache_key, &sticker_url_clone).await {
//                 log::error!("Failed to cache sticker: {}", e);
//             }
//             });

//             // Save interaction for each sticker
//             save_interaction(&db, user_id, &input_text, &detected_emotion, sticker_url).await;
//         }

//         // Return all sticker URLs
//         HttpResponse::Ok().json(RecommendResponse {
//             detected_emotion,
//             sticker_urls,
//         })
//     } else {
//         HttpResponse::BadGateway().body("No stickers returned by service")
//     }
// }

// async fn save_interaction(db: &web::Data<crate::structs::database_structs::DatabaseConnection>, user_id: Uuid, input_text: &str, emotion: &str, sticker_url: &str) {
//     if let Err(e) = db.save_interaction(user_id, input_text, emotion, sticker_url).await {
//         log::error!("Failed to save interaction: {}", e);
//     }
//     if let Err(e) = db.update_sticker_metrics(user_id, sticker_url).await {
//         log::error!("Failed to update sticker metrics: {}", e);
//     }
// }

// pub fn init_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(find_sticker);
// }

// async fn download_gif(client: &Client, url: &str) -> Result<Vec<u8>, reqwest::Error> {
//     let response = client.get(url).send().await?;
//     let bytes = response.bytes().await?;
//     Ok(bytes.to_vec())
// }

// fn compress_gif(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
//     let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
//     encoder.write_all(data)?;
//     encoder.finish()
// }

// fn decompress_gif(compressed_data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
//     let mut decoder = ZlibDecoder::new(compressed_data);
//     let mut decompressed_data = Vec::new();
//     decoder.read_to_end(&mut decompressed_data)?;
//     Ok(decompressed_data)
// }

use crate::middleware::auth::AuthData;
use crate::structs::database_structs::{RecommendRequest, RecommendResponse, Sticker};
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use reqwest::Client;
use serde_json::Value;
use std::io::{Read, Write};
use uuid::Uuid;

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

    log::info!(
        "User {} (ID: {}) requested sticker for text: {}",
        username,
        user_id,
        input_text
    );
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
        reqwest::StatusCode::OK => match emotion_response.json::<Value>().await {
            Ok(json) => json["detected_emotion"]
                .as_str()
                .unwrap_or("neutral")
                .to_string(),
            Err(e) => {
                log::error!("Failed to parse emotion response: {}", e);
                return HttpResponse::InternalServerError().body("Emotion detection failed");
            }
        },
        _ => {
            log::error!(
                "Emotion service returned error: {}",
                emotion_response.status()
            );
            return HttpResponse::InternalServerError().body("Emotion service unavailable");
        }
    };

    // Check Redis cache for sticker
    let cache_key = format!("sticker:{}", detected_emotion);
    if let Ok(Some((emotion, sticker_id, sticker_data))) = db.get_cached_sticker(&cache_key).await {
        // Save interaction for cached sticker
        let decompressed_data = match decompress_gif(&sticker_data) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to decompress cached sticker data: {}", e);
                return HttpResponse::InternalServerError()
                    .body("Failed to decompress cached sticker");
            }
        };
        let base64_data = general_purpose::STANDARD.encode(&decompressed_data);
        let data_url = format!("data:image/gif;base64,{}", base64_data);
        let interaction_id = match db
            .save_interaction(user_id, &input_text, &emotion, &data_url)
            .await
        {
            Ok(id) => id,
            Err(e) => {
                log::error!("Failed to save interaction: {}", e);
                return HttpResponse::InternalServerError().body("Failed to save interaction");
            }
        };
        // Re-save the cached sticker data to interaction_stickers
        if let Err(e) = db.save_sticker(interaction_id, &sticker_data).await {
            log::error!("Failed to save cached sticker: {}", e);
        }
        return HttpResponse::Ok().json(RecommendResponse {
            detected_emotion: emotion,
            stickers: vec![Sticker {
                id: sticker_id,
                data: data_url,
            }],
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
        reqwest::StatusCode::OK => match sticker_response.json::<Vec<Value>>().await {
            Ok(results) => results,
            Err(e) => {
                log::error!("Failed to parse sticker response: {}", e);
                return HttpResponse::InternalServerError().body("Sticker search failed");
            }
        },
        _ => {
            log::error!(
                "Sticker service returned error: {}",
                sticker_response.status()
            );
            return HttpResponse::InternalServerError().body("Sticker service unavailable");
        }
    };

    if sticker_results.is_empty() {
        return HttpResponse::BadGateway().body("No stickers returned by service");
    }

    // Save the interaction
    let mut stickers = Vec::new();
    let mut first_sticker_data = None;
    let mut first_data_url = String::new();

    for item in sticker_results.iter().take(3) {
        if let Some(url) = item["url"].as_str() {
            // Download the GIF
            let gif_data = match download_gif(&client, url).await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("Failed to download GIF from {}: {}", url, e);
                    continue;
                }
            };

            // Compress the GIF data
            let compressed_data = match compress_gif(&gif_data) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("Failed to compress GIF data: {}", e);
                    continue;
                }
            };

            // Encode original GIF data as base64
            let base64_data = general_purpose::STANDARD.encode(&gif_data);
            let data_url = format!("data:image/gif;base64,{}", base64_data);

            // Save interaction for the first sticker
            let interaction_id = if first_data_url.is_empty() {
                first_data_url = data_url.clone();
                match db
                    .save_interaction(user_id, &input_text, &detected_emotion, &data_url)
                    .await
                {
                    Ok(id) => id,
                    Err(e) => {
                        log::error!("Failed to save interaction: {}", e);
                        return HttpResponse::InternalServerError()
                            .body("Failed to save interaction");
                    }
                }
            } else {
                // For additional stickers, create new interactions or just save to interaction_stickers
                match db
                    .save_interaction(user_id, &input_text, &detected_emotion, &data_url)
                    .await
                {
                    Ok(id) => id,
                    Err(e) => {
                        log::error!("Failed to save interaction: {}", e);
                        continue;
                    }
                }
            };

            // Save compressed data to interaction_stickers
            let sticker_id = match db.save_sticker(interaction_id, &compressed_data).await {
                Ok(id) => id,
                Err(e) => {
                    log::error!("Failed to save sticker to database: {}", e);
                    continue;
                }
            };

            // Store first sticker for caching
            if first_sticker_data.is_none() {
                first_sticker_data = Some((sticker_id, compressed_data.clone()));
            }

            // Update sticker metrics using the original URL
            if let Err(e) = db.update_sticker_metrics(user_id, url).await {
                log::error!("Failed to update sticker metrics: {}", e);
            }

            stickers.push(Sticker {
                id: sticker_id,
                data: data_url,
            });
        }
    }

    // Cache the first sticker
    if let Some((sticker_id, compressed_data)) = first_sticker_data {
        let db_clone = db.clone();
        let cache_key_clone = cache_key.clone();
        let emotion_clone = detected_emotion.clone();
        actix_web::rt::spawn(async move {
            if let Err(e) = db_clone
                .cache_sticker(
                    &cache_key_clone,
                    &emotion_clone,
                    sticker_id,
                    &compressed_data,
                )
                .await
            {
                log::error!("Failed to cache sticker: {}", e);
            }
        });
    }

    if stickers.is_empty() {
        return HttpResponse::BadGateway().body("No stickers could be processed");
    }

    HttpResponse::Ok().json(RecommendResponse {
        detected_emotion,
        stickers,
    })
}

// async fn save_interaction(
//     db: &web::Data<crate::structs::database_structs::DatabaseConnection>,
//     user_id: Uuid,
//     input_text: &str,
//     emotion: &str,
//     sticker_url: &str,
// )
// {
//     if let Err(e) = db.save_interaction(user_id, input_text, emotion, sticker_url).await {
//         log::error!("Failed to save interaction: {}", e);
//     }
// }

async fn download_gif(client: &Client, url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

fn compress_gif(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

fn decompress_gif(compressed_data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_sticker);
}
