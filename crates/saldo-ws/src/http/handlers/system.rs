use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

use crate::http::responses::ApiSuccess;

pub async fn health() -> impl IntoResponse {
    axum::Json(json!({ "status" : "UP" }))
}

pub async fn version() -> ApiSuccess<VersionResponseData> {
    ApiSuccess::json(StatusCode::OK, VersionResponseData { version: "26005" })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct VersionResponseData {
    version: &'static str,
}
