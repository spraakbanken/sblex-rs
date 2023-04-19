use crate::routes::system;
use axum::routing::get;
use axum::Router;

pub fn app() -> Router {
    Router::new().route("/health", get(system::health))
}
