use ntex::web;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::controllers::video_controller::VideoController;
use crate::repositories::video_repository::VideoRepository;
use crate::services::video_service::VideoService;

mod api;
mod app;
mod config;
mod controllers;
mod db;
mod entity;
mod error;
mod repositories;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    // Load configuration
    let config = config::Config::from_env();
    info!("Starting server with config: {:?}", config);
    
    // Initialize database
    let db = db::init_db().await;
    info!("Database initialized");
    
    let addr = format!("{}:{}", config.server_host, config.server_port);
    info!("Starting server at: {}", addr);

    // Initialize repository, service and controller
    let repository = VideoRepository::new(db.clone());
    let service = VideoService::new(repository);
    let controller = VideoController::new(service);

    web::HttpServer::new(move || {
        web::App::new()
            .state(db.clone())
            .configure(|cfg| controllers::video_controller::config(cfg, controller.clone()))
    })
    .bind(&addr)?
    .run()
    .await
}
