use axum::{
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub nome: String,
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn is_up() -> &'static str {
    "Server Up"
}

pub async fn get_user() -> impl IntoResponse {
    let user = User {
        id: 1,
        nome: "Jonathan Carvalho".to_string(),
    };
    Json(user)
}
