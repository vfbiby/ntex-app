mod api;
mod app;

use ntex::web;
use crate::app::config_app;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().configure(config_app))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

