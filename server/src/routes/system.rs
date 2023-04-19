use axum::response::IntoResponse;
use serde_json::json;

pub async fn health() -> impl IntoResponse {
    axum::Json(json!({ "status" : "UP" }))
}

pub async fn version() -> impl IntoResponse {
    axum::Json(json!({"version": "26005"}))
}
