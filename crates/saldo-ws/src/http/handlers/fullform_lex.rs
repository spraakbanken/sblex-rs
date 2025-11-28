use askama::Template;
use axum::extract::State;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use axum::response::{Html, IntoResponse};

use sblex_services::models::fullform::Fullform;
use sblex_services::models::fullform_lex::FullformLex;
use sblex_services::models::lemma::Lemma;
use sblex_services::models::lexeme::Lexeme;
use sblex_services::models::lookup::SaldoOrLemmaId;
use sblex_services::ports::SblexService;

use crate::http::error::ApiError;
use crate::http::extractors::Path;
use crate::http::responses::{ApiSuccessJson, ApiSuccessRaw};
use crate::http::AppState;

pub async fn fullform_lex_json<S: SblexService>(
    State(state): State<AppState<S>>,
    Path(segment): Path<String>,
) -> Result<ApiSuccessJson<Vec<FullformLex>>, ApiError> {
    state
        .sblex_service
        .fullform_lex_query(segment.as_str())
        .map_err(ApiError::from)
        .map(|json_data| ApiSuccessJson::new(StatusCode::OK, json_data))
}
