mod api;

use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().route("/", web::get().to(api::index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
