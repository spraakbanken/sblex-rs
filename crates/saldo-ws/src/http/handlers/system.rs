use axum::http::StatusCode;

use crate::http::responses::ApiSuccessJson;

pub async fn health() -> ApiSuccessJson<StatusResponseData> {
    ApiSuccessJson::new(StatusCode::OK, StatusResponseData { status: "UP" })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct StatusResponseData {
    status: &'static str,
}

pub async fn version() -> ApiSuccessJson<VersionResponseData> {
    ApiSuccessJson::new(StatusCode::OK, VersionResponseData { version: "26005" })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct VersionResponseData {
    version: &'static str,
}
