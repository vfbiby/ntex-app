use ntex::http::StatusCode;
use ntex::web::test::TestRequest;

mod common;
use common::{assert_body, assert_header, assert_status};

mod video_tests {
    use super::*;

    #[ntex::test]
    async fn test_videos_endpoint_returns_200() {
        assert_status(TestRequest::get().uri("/videos"), StatusCode::OK).await;
    }

    #[ntex::test]
    async fn test_videos_endpoint_returns_json() {
        assert_header(
            TestRequest::get().uri("/videos"),
            "content-type",
            "application/json",
        )
        .await;
    }

    #[ntex::test]
    async fn test_empty_videos_returns_empty_array() {
        assert_body(
            TestRequest::get().uri("/videos"),
            b"{\"videos\":[],\"total\":0,\"page\":1,\"per_page\":10,\"total_pages\":0}"
        ).await;
    }
}
