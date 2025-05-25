use aula_pipeline::handlers::{get_user, hello_world, is_up};
use axum::response::IntoResponse;
use hyper::body::to_bytes;
use serde_json::{json, Value};

#[tokio::test]
async fn test_get_user() {
    let response = get_user().await.into_response();
    let body = to_bytes(response.into_body()).await.unwrap();
    let json_body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json_body, json!({
        "id": 1,
        "nome": "Jonathan Carvalho"
    }));
}

#[tokio::test]
async fn test_hello_world() {
    let response = hello_world().await;
    assert_eq!(response, "Hello, World!");
}

#[tokio::test]
async fn test_is_up() {
    let response = is_up().await;
    assert_eq!(response, "Server Up");
}
