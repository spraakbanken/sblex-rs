use std::time::Duration;

use axum::Router;

use crate::state::AppState;

use self::morph::morph_routes;

use tower_http::timeout::TimeoutLayer;


mod morph;

pub fn create_app(state: AppState) -> Router {
    // let mut api = OpenApi::default();

    // let app = ApiRouter::new()
    //     .nest_api_service("/docs", docs_routes(state.clone()))
    //     .finish_api_with(&mut api, api_docs)
    //     .layer(Extension(Arc::new(api)))
    //     .with_state(state);

    Router::new().nest("/morph", morph_routes(state)).layer((
        // TraceLayer::new_for_http().make_span_with(make_span),
        TimeoutLayer::new(Duration::from_secs(10)),
        // include trace context as header into the response
        // OtelInResponseLayer::default(),
        //start OpenTelemetry trace on incoming request
        // OtelAxumLayer::default(),
    ))
}


