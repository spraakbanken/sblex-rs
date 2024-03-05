use axum::Router;

use crate::state::AppState;

use self::morph::morph_routes;

mod morph;

pub fn create_app(state: AppState) -> Router {
    // let mut api = OpenApi::default();

    // let app = ApiRouter::new()
    //     .nest_api_service("/docs", docs_routes(state.clone()))
    //     .finish_api_with(&mut api, api_docs)
    //     .layer(Extension(Arc::new(api)))
    //     .with_state(state);

    Router::new().nest("/morph", morph_routes(state))
}

// fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
//     api.title("FM server").summary("FM server")
// }
