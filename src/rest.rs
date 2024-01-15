use crate::db::{all_brackets, Bracket};
use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use sqlx::MySqlPool;

pub fn bracket_service() -> Router {
    Router::new().route("/", get(get_all_brackets_handler))
}

async fn get_all_brackets_handler(
    Extension(conn): Extension<MySqlPool>,
) -> Result<Json<Vec<Bracket>>, StatusCode> {
    match all_brackets(&conn).await {
        Ok(brackets) => {
            println!("Retrieved Brackets sucessfully.");
            Ok(Json(brackets))
        }
        Err(err) => {
            println!("Failed to get Brackets {:?}", err);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
