use axum::{extract::rejection::QueryRejection, http::StatusCode, response::IntoResponse, Json};
use sblex_services::models::lookup::LookupError;

use crate::http::responses::ApiResponseBody;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    InternalServerError(String),
    QueryRejection(QueryRejection),
}

impl From<QueryRejection> for ApiError {
    fn from(value: QueryRejection) -> Self {
        ApiError::QueryRejection(value)
    }
}
impl From<LookupError> for ApiError {
    fn from(cause: LookupError) -> Self {
        let source_chain = error_source_chain(&cause);
        tracing::error!(
            error.msg = %cause,
            error.details = ?cause,
            error.source_chain = %source_chain,
            "An error occurred during request handling"
        );
        ApiError::InternalServerError("Internal server error".to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponseBody::new_error(StatusCode::NOT_FOUND, message)),
            )
                .into_response(),
            QueryRejection(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message.to_string(),
                )),
            )
                .into_response(),
        }
    }
}
/// The canonical representation for the value in [`ERROR_SOURCE_CHAIN`].
pub fn error_source_chain(e: &(dyn std::error::Error + Send + Sync + 'static)) -> String {
    _error_source_chain(e)
}

fn _error_source_chain(e: &(dyn std::error::Error + Send + Sync + 'static)) -> String {
    use std::fmt::Write as _;

    let mut chain = String::new();
    let mut source = e.source();
    while let Some(s) = source {
        let _ = writeln!(chain, "- {}", s);
        source = s.source();
    }
    chain
}
