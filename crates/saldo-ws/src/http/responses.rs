use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

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
