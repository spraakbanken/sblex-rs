use askama::Template;
use axum::extract::State;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use axum::response::{Html, IntoResponse};

use sblex_services::models::lemma::Lemma;
use sblex_services::models::lexeme::Lexeme;
use sblex_services::models::lookup::SaldoOrLemmaId;
use sblex_services::ports::SblexService;

use crate::http::error::ApiError;
use crate::http::extractors::Path;
use crate::http::responses::{ApiSuccessJson, ApiSuccessRaw};
use crate::http::AppState;

pub async fn fullform_json<S: SblexService>(
    State(state): State<AppState<S>>,
    Path(fragment): Path<String>,
) -> Result<ApiSuccessRaw<Vec<u8>>, ApiError> {
    state
        .sblex_service
        .lookup_morph_with_cont(fragment.as_str())
        .map_err(ApiError::from)
        .map(|json_data| ApiSuccessRaw::new(StatusCode::OK, "application/json", json_data))
}
