use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::web;
use actix_web::{self, middleware::Logger, App, HttpServer, web::Data};
use crate::configs::env_load::_load_envs as load_envs;
use crate::structs::database_structs::DatabaseConnection;
use crate::routes;
use crate::middleware::auth::AuthConfig;
use crate::middleware;
use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_web::main]
pub async fn init_app() -> std::io::Result<()> {
    // Setting up the Governor
    let governor = GovernorConfigBuilder::default()
        .seconds_per_request(30)
        .burst_size(30)
        .finish()
        .unwrap();
    
        let db = DatabaseConnection::new().await.expect("Failed to connect to database");
        db.init_schema().await.expect("Failed to initialize database schema");
        let auth_config = AuthConfig::new();
    
    // Start the server
    HttpServer::new(
        move || {
            let auth = HttpAuthentication::bearer(middleware::validate::jwt_middleware);
            App::new()
                .wrap(Logger::default())

                .wrap(Governor::new(&governor))

                .wrap(middleware::cors_mgt::handle_cors())

                .app_data(Data::new(db.clone()))
                
                .app_data(web::Data::new(auth_config.clone()))
                
                .configure(routes::health::init_routes)
                
                .configure(routes::auth::init_routes)

                .service(
                    web::scope("/user")
                        .wrap(auth.clone())
                        .configure(routes::user::init_routes),
                )
                .service(
                    web::scope("/admin")
                        .wrap(auth)
                        .configure(routes::admin::init_routes),
                )
        }
    )
    .workers(4)
    .bind((load_envs().0, load_envs().1))?
    .run()
    .await
}