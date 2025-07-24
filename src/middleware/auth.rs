use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims  {
    // pieces of information (key-value pairs) included in the payload section of a JWT
    pub sub: String,  // User ID
    pub role: String, // "user" or "admin"
    pub exp: usize,   // Expiration time
}

#[derive(Clone)]
pub struct AuthConfig {
    secret: String,
}

impl AuthConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
        Self { secret }
    }
}

    pub fn create_token(user_id: &str, role:&str , config: &AuthConfig) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(), // or "admin" based on your logic
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn validate_token(token: &str, config: &AuthConfig) -> Result<Claims, jsonwebtoken::errors::Error> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(decoded.claims)
    }
    
    
    
    
    
    
    
    
    
    
    
#[derive(Clone)]
#[allow(dead_code)]
pub struct AuthData {
    pub id: String,
    pub role: String,
}

impl AuthData {
    pub fn new(id: String, role: String) -> Self {
        AuthData { id, role }
    }
}