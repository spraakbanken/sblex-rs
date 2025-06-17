use axum::extract::rejection::QueryRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sblex_services::models::lookup::LookupError;

#[derive(Debug, Clone)]
pub struct ApiSuccessJson<T: serde::Serialize + PartialEq>(StatusCode, Json<T>);

impl<T: serde::Serialize + PartialEq> ApiSuccessJson<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self(status, Json(data))
    }
}

impl<T: serde::Serialize + PartialEq> IntoResponse for ApiSuccessJson<T> {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
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
