mod api;
mod app;
mod db;

use ntex::web;
use crate::app::config_app;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let db = db::init_db().await;
    
    web::HttpServer::new(move || {
        web::App::new()
            .state(db.clone())
            .configure(config_app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
