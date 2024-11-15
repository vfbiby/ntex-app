use ntex::http::Request;
use ntex::{web, Pipeline, Service};
use ntex::web::{test, Error, WebResponse};

async fn init_test_service() -> Pipeline<impl Service<Request, Response=WebResponse, Error=Error> + Sized> {
    test::init_service(web::App::new().route("/", web::get().to(ntex_api::api::index))).await
}