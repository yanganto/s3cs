use common::MockServer;
use reqwest::{get, StatusCode};

mod common;

#[tokio::test]
async fn test_basic_operations() {
    let _s = MockServer::new().await;
    let r = get("http://127.0.0.1:3333")
        .await
        .expect("admin service should work");

    assert_eq!(r.status(), StatusCode::OK);
}
