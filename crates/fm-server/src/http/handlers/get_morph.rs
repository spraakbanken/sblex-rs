use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use sblex_services::Morphology;
use tracing::instrument;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

use crate::http::AppState;

#[derive(serde::Deserialize)]
pub struct Params {
    fragment: String,
    n: Option<usize>,
}

#[instrument(name = "get-saldo-morph", skip(state))]
pub async fn get_saldo_morph<SM: Morphology>(
    State(state): State<AppState<SM>>,
    Path(Params { fragment, n }): Path<Params>,
) -> impl IntoResponse {
    tracing::debug!(?fragment, "get_morph called");
    let trace_id = find_current_trace_id();
    tracing::debug!(?trace_id, "again");
    let json_data = state
        .saldo_morphology
        .read()
        .await
        .lookup_with_state(&fragment, n.unwrap_or(0))
        .map(|s| s.to_owned());
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    if let Some(json_data) = json_data {
        (StatusCode::OK, headers, json_data).into_response()
    } else {
        eprintln!("didn't find {}", fragment);
        (
            StatusCode::NOT_FOUND,
            headers,
            format!("{{\"error\":\"Not found: {fragment}{n:?}\""),
        )
            .into_response()
    }
}
