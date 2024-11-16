use ntex::http::{Request, StatusCode};
use ntex::util::Bytes;
use ntex::web::{test, test::TestRequest, Error, WebResponse};
use ntex::{web, Pipeline, Service};
use ntex_api::app::config_app;
use sea_orm::{Database, DatabaseConnection, Schema, ConnectionTrait};

pub async fn init_test_service(
    db: DatabaseConnection,
) -> Pipeline<impl Service<Request, Response = WebResponse, Error = Error> + Sized> {
    test::init_service(
        web::App::new()
            .state(db)
            .configure(config_app)
    ).await
}

// 辅助函数：验证响应状态码
pub async fn assert_status(req: TestRequest, expected_status: StatusCode) {
    let db = setup_database().await;
    let app = init_test_service(db).await;
    let resp = test::call_service(&app, req.to_request()).await;
    assert_eq!(resp.status(), expected_status);
}

// 辅助函数：验证响应头
pub async fn assert_header(req: TestRequest, header_name: &str, expected_value: &str) {
    let db = setup_database().await;
    let app = init_test_service(db).await;
    let resp = test::call_service(&app, req.to_request()).await;
    let headers = resp.headers();
    assert_eq!(
        headers.get(header_name).unwrap().to_str().unwrap(),
        expected_value
    );
}

// 辅助函数：验证响应体
pub async fn assert_body(req: TestRequest, expected_body: &[u8]) {
    let db = setup_database().await;
    let app = init_test_service(db).await;
    let resp = test::call_service(&app, req.to_request()).await;
    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::copy_from_slice(expected_body));
}

pub async fn setup_database() -> DatabaseConnection {
    let database_url = "sqlite::memory:";
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize the database schema
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    let mut table = schema.create_table_from_entity(ntex_api::db::Entity);
    let stmt = table.if_not_exists();
    db.execute(backend.build(stmt))
        .await
        .expect("Failed to create table");

    db
}
