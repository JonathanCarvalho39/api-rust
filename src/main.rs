// main.rs
mod handlers;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    server::run_server().await;
}