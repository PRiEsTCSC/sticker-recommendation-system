use crate::middleware::auth::{validate_token, AuthConfig, AuthData};
use crate::structs::database_structs::DatabaseConnection;
use actix_web::{dev::ServiceRequest, error::Error as ActixError, web, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::pin::Pin;

pub fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Pin<Box<dyn futures::Future<Output = Result<ServiceRequest, (ActixError, ServiceRequest)>>>> {
    Box::pin(async move {
        println!("🔐 JWT MIDDLEWARE CALLED for path: {}", req.path());
        log::info!("Processing JWT middleware for path: {}", req.path());

        // Extract app data early to release immutable borrow
        let db = match req.app_data::<web::Data<DatabaseConnection>>() {
            Some(db) => db.clone(),
            None => {
                log::error!("DatabaseConnection missing in app data");
                return Err((
                    actix_web::error::ErrorInternalServerError("DatabaseConnection missing"),
                    req,
                ));
            }
        };
        let auth_config = match req.app_data::<web::Data<AuthConfig>>() {
            Some(config) => config.clone(),
            None => {
                log::error!("AuthConfig missing in app data");
                return Err((
                    actix_web::error::ErrorInternalServerError("AuthConfig missing"),
                    req,
                ));
            }
        };

        // Validate JWT token first
        log::info!("Validating token: {}", credentials.token());
        let claims = match validate_token(credentials.token(), auth_config.get_ref()) {
            Ok(claims) => {
                log::info!("Token validated, sub: {}", claims.sub);
                claims
            }
            Err(e) => {
                log::warn!("Invalid JWT token: {}", e);
                return Err((actix_web::error::ErrorUnauthorized("Invalid token"), req));
            }
        };

        // Validate session
        log::info!("Validating session for token: {}", credentials.token());
        let session = match db.validate_session(credentials.token()).await {
            Ok(Some(session)) => {
                log::info!(
                    "Session validated, user_id: {:?}, admin_id: {:?}",
                    session.user_id,
                    session.admin_id
                );
                session
            }
            Ok(None) => {
                log::warn!("Invalid or expired session for {}", claims.sub);
                return Err((
                    actix_web::error::ErrorUnauthorized("Invalid or expired session"),
                    req,
                ));
            }
            Err(e) => {
                log::error!("Database error during session validation: {}", e);
                return Err((
                    actix_web::error::ErrorInternalServerError("Database error"),
                    req,
                ));
            }
        };

        // Get user ID from session or claims
        let user_id = if let Some(session_user_id) = session.user_id {
            session_user_id
        } else {
            // Parse user ID from claims if not in session
            match uuid::Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(e) => {
                    log::error!("Invalid user ID format in claims: {}", e);
                    return Err((
                        actix_web::error::ErrorBadRequest("Invalid user ID format"),
                        req,
                    ));
                }
            }
        };

        // Verify user exists in database
        log::info!("Checking if user {} exists", user_id);
        let user = match db.get_user_by_id(user_id).await {
            Ok(Some(user)) => {
                log::info!("Found user: id={}", user.id);
                user
            }
            Ok(None) => {
                log::warn!("User {} not found in users table", user_id);
                return Err((actix_web::error::ErrorUnauthorized("User not found"), req));
            }
            Err(e) => {
                log::error!("Database error while fetching user {}: {}", user_id, e);
                return Err((
                    actix_web::error::ErrorInternalServerError("Database error"),
                    req,
                ));
            }
        };

        // Verify token's sub matches user's ID
        if claims.sub != user.id.to_string() {
            log::warn!(
                "Token sub {} does not match user ID {}",
                claims.sub,
                user.id
            );
            return Err((
                actix_web::error::ErrorUnauthorized("Token does not match user"),
                req,
            ));
        }

        // Verify session's user_id matches user's ID (if session has user_id)
        if let Some(session_user_id) = session.user_id {
            if session_user_id != user.id {
                log::warn!(
                    "Session user_id {} does not match user ID {}",
                    session_user_id,
                    user.id
                );
                return Err((
                    actix_web::error::ErrorUnauthorized("Session does not match user"),
                    req,
                ));
            }
        }

        // Enforce role-based access (only allow users for /v1/sticker/find)
        let role = "user";
        if req.path().starts_with("/admin") {
            log::warn!("Non-admin {} attempted admin access", claims.sub);
            return Err((
                actix_web::error::ErrorForbidden("Admin access required"),
                req,
            ));
        }

        // Attach AuthData to request extensions
        log::info!("Attaching AuthData: id={}, role={}", user.id, role);
        req.extensions_mut()
            .insert(AuthData::new(user.id.to_string(), role.to_string()));

        // Return the modified request
        log::info!("Middleware completed successfully for path: {}", req.path());
        Ok(req)
    })
}
