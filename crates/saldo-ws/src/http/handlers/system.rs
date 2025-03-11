use axum::http::StatusCode;

use crate::http::responses::ApiSuccess;

pub async fn health() -> ApiSuccess<StatusResponseData> {
    ApiSuccess::json(StatusCode::OK, StatusResponseData { status: "UP" })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct StatusResponseData {
    status: &'static str,
}

pub async fn version() -> ApiSuccess<VersionResponseData> {
    ApiSuccess::json(StatusCode::OK, VersionResponseData { version: "26005" })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct VersionResponseData {
    version: &'static str,
}
