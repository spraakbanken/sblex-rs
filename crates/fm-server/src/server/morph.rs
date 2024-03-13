use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing, Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tracing::instrument;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

use crate::state::AppState;

pub fn morph_routes(state: AppState) -> Router {
    Router::new()
        .route("/:fragment/:n", routing::get(get_morph))
        // include trace context as header into the response
        .layer(OtelInResponseLayer::default())
        //start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default())
        .with_state(state)
}

#[derive(serde::Deserialize)]
struct Params {
    fragment: String,
    n: Option<usize>,
}

#[instrument(name = "get-morph", skip(state))]
async fn get_morph(
    State(state): State<AppState>,
    Path(Params { fragment, n }): Path<Params>,
) -> impl IntoResponse {
    tracing::debug!(?fragment, "get_morph called");
    let trace_id = find_current_trace_id();
    tracing::debug!(?trace_id, "again");
    let json_data = state
        .morphology
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

// fn get_morph_docs(op: TransformOperation) -> TransformOperation {
//     op.description("Create a new incomplete Todo item.")
//         .response::<200, Json<TodoCreated>>()
// }
