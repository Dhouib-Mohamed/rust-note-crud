use axum::Json;
use axum::response::IntoResponse;
use crate::utils::log::debug;

pub async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    debug("Health check successful");

    Json(json_response)
}