// routes.rs
use axum::{Router, routing::get};
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(handlers::hello_world))
        .route("/up", get(handlers::is_up))
        .route("/user", get(handlers::get_user))
        .route("/actuator/health", get(handlers::health_check))
}