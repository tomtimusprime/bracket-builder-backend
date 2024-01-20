use anyhow::Result;
use axum::{Extension, Router};
use std::env;
use tokio::net::TcpListener;
mod db;
mod rest;
use db::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let connection_pool = init_db().await?;

    let app = Router::new()
        .nest_service("/brackets", rest::bracket_service())
        .layer(Extension(connection_pool));

    let addr = env::var("LISTEN_ADDR").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3002".to_string());

    let listener = TcpListener::bind(format!("{addr}:{port}")).await?;
    let addr = listener.local_addr()?;
    println!("Listening on {:?}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
