use actix_web::{error::Error as ActixError, web, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::pin::Pin;

use crate::structs::database_structs::DatabaseConnection;

use super::auth::{validate_token, AuthConfig};

// Assuming DatabaseConnection and AuthConfig are defined elsewhere
pub fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Pin<Box<dyn futures::Future<Output = Result<ServiceRequest, (ActixError, ServiceRequest)>>>> {
    Box::pin(async move {
        // Extract app data
        let db = req.app_data::<web::Data<DatabaseConnection>>()
            .expect("DatabaseConnection missing");
        let auth_config = req.app_data::<web::Data<AuthConfig>>()
            .expect("AuthConfig missing");

        // Validate JWT token
        let claims = match validate_token(credentials.token(), auth_config.get_ref()) {
            Ok(claims) => claims,
            Err(e) => {
                log::warn!("Invalid JWT token: {}", e);
                return Err((actix_web::error::ErrorUnauthorized("Invalid token"), req));
            }
        };

        // Validate session
        let session = match db.validate_session(credentials.token()).await {
            Ok(Some(session)) => session,
            Ok(None) => {
                log::warn!("Invalid or expired session for {}", claims.sub);
                return Err((
                    actix_web::error::ErrorUnauthorized("Invalid or expired session"),
                    req,
                ));
            }
            Err(_) => {
                return Err((
                    actix_web::error::ErrorInternalServerError("Database error"),
                    req,
                ));
            }
        };

        // Determine role and ID
        let (role, id) = match (session.user_id, session.admin_id) {
            (Some(user_id), None) => ("user", user_id.to_string()),
            (None, Some(admin_id)) => ("admin", admin_id.to_string()),
            _ => {
                return Err((
                    actix_web::error::ErrorInternalServerError("Invalid session"),
                    req,
                ))
            }
        };

        // Enforce role-based access
        if req.path().starts_with("/admin") && role != "admin" {
            log::warn!("Non-admin {} attempted admin access", claims.sub);
            return Err((actix_web::error::ErrorForbidden("Admin access required"), req));
        }
        if req.path().starts_with("/user") && role != "user" {
            log::warn!("Non-user {} attempted user access", claims.sub);
            return Err((actix_web::error::ErrorForbidden("User access required"), req));
        }

        // Attach role and ID to request extensions
        actix_web::HttpMessage::extensions_mut(&req).insert(role.to_string());
        actix_web::HttpMessage::extensions_mut(&req).insert(id);

        // Return the modified request
        Ok(req)
    })
}