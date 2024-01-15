use anyhow::Result;
use axum::{Extension, Router};
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

    let listener = TcpListener::bind("0.0.0.0:3002").await?;
    let addr = listener.local_addr()?;
    println!("Listening on {:?}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
