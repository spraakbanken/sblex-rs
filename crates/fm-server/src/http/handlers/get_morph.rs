use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use sblex_services::{models::lookup::LookupError, ports::Morphology};
use tracing::instrument;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

use crate::http::AppState;

#[derive(Debug, Clone)]
pub struct ApiSuccess<T: PartialEq + IntoResponse>(StatusCode, T);

impl<T: PartialEq + IntoResponse> ApiSuccess<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, data)
    }
}

impl<T: PartialEq + IntoResponse> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> axum::response::Response {
        (self.0, [(header::CONTENT_TYPE, "application/json")], self.1).into_response()
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ApiResponseBody<T: PartialEq + serde::Serialize> {
    status_code: u16,
    data: T,
}

// impl<T: PartialEq + serde::Serialize> ApiResponseBody<T> {
//     pub fn new(status_code: StatusCode, data: T) -> Self {
//         Self {
//             status_code: status_code.as_u16(),
//             data,
//         }
//     }
// }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    // UnprocessableEntity(String),
    NotFound(String),
}

impl From<LookupError> for ApiError {
    fn from(value: LookupError) -> Self {
        match value {
            LookupError::Unknown(cause) => {
                let source_chain = error_source_chain(cause.as_ref());
                tracing::error!(
                    error.msg = %cause,
                    error.details = ?cause,
                    error.source_chain = %source_chain,
                    "An error occurred during request handling"
                );
                Self::InternalServerError("Internal server error".to_string())
            }
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
            // UnprocessableEntity(message) => (
            //     StatusCode::UNPROCESSABLE_ENTITY,
            //     Json(ApiResponseBody::new_error(
            //         StatusCode::UNPROCESSABLE_ENTITY,
            //         message,
            //     )),
            // )
            //     .into_response(),
            NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponseBody::new_error(StatusCode::NOT_FOUND, message)),
            )
                .into_response(),
        }
    }
}

#[instrument(name = "get-saldo-morph", skip(state))]
pub async fn get_saldo_morph<SM: Morphology>(
    State(state): State<AppState<SM>>,
    Path(fragment): Path<String>,
) -> Result<ApiSuccess<Vec<u8>>, ApiError> {
    tracing::debug!(?fragment, "get_saldo_morph called");
    let trace_id = find_current_trace_id();
    tracing::debug!(?trace_id, "again");
    let json_data = state
        .saldo_morphology
        .read()
        .await
        .lookup(&fragment)
        .map_err(ApiError::from)?;
    if let Some(json_data) = json_data {
        Ok(ApiSuccess::new(StatusCode::OK, json_data))
    } else {
        Err(ApiError::NotFound(fragment.to_string()))
    }
}

#[instrument(name = "get-saldo-morph-w-cont", skip(state))]
pub async fn get_saldo_morph_w_cont<SM: Morphology>(
    State(state): State<AppState<SM>>,
    Path(fragment): Path<String>,
) -> Result<ApiSuccess<Vec<u8>>, ApiError> {
    tracing::debug!(?fragment, "get_saldo_morph_w_cont called");
    let trace_id = find_current_trace_id();
    tracing::debug!(?trace_id, "again");
    state
        .saldo_morphology
        .read()
        .await
        .lookup_with_cont(&fragment)
        .map_err(ApiError::from)
        .map(|data| ApiSuccess::new(StatusCode::OK, data))
}
