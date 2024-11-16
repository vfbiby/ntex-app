mod api;
mod app;
mod db;
mod config;
mod error;
mod entity;

use ntex::web;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let _ = FmtSubscriber::builder()
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

    web::HttpServer::new(move || {
        web::App::new()
            .state(db.clone())
            .configure(app::config_app)
    })
    .bind(&addr)?
    .run()
    .await
}
