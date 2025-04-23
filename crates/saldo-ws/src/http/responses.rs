use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sblex_services::models::lookup::LookupError;

#[derive(Debug, Clone)]
pub enum ApiSuccess<T: serde::Serialize + PartialEq> {
    Json(StatusCode, Json<T>),
}

impl<T: serde::Serialize + PartialEq> ApiSuccess<T> {
    pub fn json(status: StatusCode, data: T) -> Self {
        Self::Json(status, Json(data))
    }
}

impl<T: serde::Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Json(status, data) => (status, data).into_response(),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    InternalServerError(String),
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

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ApiResponseBody<T: serde::Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: serde::Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ApiErrorData {
    pub message: String,
}
