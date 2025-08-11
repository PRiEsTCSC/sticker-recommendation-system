mod configs;
mod init;
mod middleware;
mod models;
mod routes;
mod structs;

use log::{error, info};
use std::env;

fn main() {
    println!("DEBUG: Current working directory: {}", env::current_dir().unwrap().display());
    println!("DEBUG: Starting logger initialization");
    log4rs::init_file("/app/log4rs.yaml", Default::default()).expect("Failed to initialize logger");
    println!("DEBUG: Logger initialized");
    info!("Starting server...");
    println!("DEBUG: Calling init_app");
    let server = init::init_app();
    println!("DEBUG: init_app returned");
    match server {
        Ok(_) => {
            info!("Server started successfully");
            println!("DEBUG: Server started successfully");
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            println!("DEBUG: Server failed: {}", e);
            std::process::exit(1); // Force non-zero exit on error
        }
    }
    println!("DEBUG: End of main");
}