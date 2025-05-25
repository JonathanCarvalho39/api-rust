// server.rs
use std::net::SocketAddr;
use crate::routes;

pub async fn run_server() {
    let app = routes::create_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Servidor rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}