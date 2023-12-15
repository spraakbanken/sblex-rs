use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::Extension;
use axum::Router;
use std::sync::Arc;

use crate::docs::docs_routes;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .with_state(state);
    app
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("FM server").summary("FM server")
}
