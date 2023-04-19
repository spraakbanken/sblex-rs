use crate::routes::system;
use crate::routes::lids;

use axum::routing::get;
use axum::Router;
use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};

pub fn app() -> Router {
    Router::new()
        .route("/lid/json/:lid", get(lids::lookup_lid_json)) // request processed inside span
        .route("/lid/xml/:lid", get(lids::lookup_lid_xml)) // request processed inside span
        .route("/lid/html/:lid", get(lids::lookup_lid_html)) // request processed inside span
        // include trace context as header into the response
        .layer(response_with_trace_layer())
        // opentelemetry_tracing_layer setup `TraceLayer`,
        // that is provided by tower-http so you have to add that as a dependency.
        .layer(opentelemetry_tracing_layer())
        .route("/health", get(system::health))
        .route("/version/json", get(system::version))
}
