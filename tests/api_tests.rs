use ntex::http::StatusCode;
use ntex::web::test::TestRequest;
use sea_orm::DatabaseConnection;

mod common;
use common::{assert_body, assert_header, assert_status, setup_database};

mod video_tests {
    use super::*;

    async fn setup() -> DatabaseConnection {
        setup_database().await
    }

    #[ntex::test]
    async fn test_videos_endpoint_returns_200() {
        let db = setup().await;
        assert_status(TestRequest::get().uri("/videos").state(db), StatusCode::OK).await;
    }

    #[ntex::test]
    async fn test_videos_endpoint_returns_json() {
        let db = setup().await;
        assert_header(
            TestRequest::get().uri("/videos").state(db),
            "content-type",
            "application/json",
        )
        .await;
    }

    #[ntex::test]
    async fn test_empty_videos_returns_empty_array() {
        let db = setup().await;
        assert_body(TestRequest::get().uri("/videos").state(db), b"[]").await;
    }
}
