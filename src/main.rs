use axum::{
    routing::get,
    Router,
};
mod rest;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {"Hello world!"}));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
