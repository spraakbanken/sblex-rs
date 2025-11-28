use std::io;
use std::sync::Arc;

use crate::http::handlers::fullform_lex;
use crate::http::handlers::fullforms;
use crate::http::handlers::lids;
use crate::http::handlers::system;

use axum::routing::get;
use axum::Router;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use sblex_services::ports::SblexService;
use tokio::{net::TcpListener, signal};

mod error;
mod extractors;
mod handlers;
mod responses;

fn app<S: SblexService>(state: AppState<S>) -> Router {
    Router::new()
        .route("/lid/json/{lid}", get(lids::lookup_lid_json::<S>)) // request processed inside span
        .route("/lid/xml/{lid}", get(lids::lookup_lid_xml)) // request processed inside span
        .route("/lid/html/{lid}", get(lids::lookup_lid_html)) // request processed inside span
        .route("/ff/json/{fragment}", get(fullforms::fullform_json::<S>))
        .route(
            "/fl/json/{segment}",
            get(fullform_lex::fullform_lex_json::<S>),
        )
        // include trace context as header into the response
        .layer(OtelInResponseLayer::default())
        // opentelemetry_tracing_layer setup `TraceLayer`,
        // that is provided by tower-http so you have to add that as a dependency.
        .layer(OtelAxumLayer::default())
        .route("/health", get(system::health))
        .route("/version/json", get(system::version))
        .with_state(state)
}

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers
struct AppState<S: SblexService> {
    sblex_service: Arc<S>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: TcpListener,
}

pub struct HttpServerConfig<'a> {
    pub port: u16,
    pub host: &'a str,
}
impl HttpServer {
    pub async fn new(
        sblex_service: impl SblexService,
        config: HttpServerConfig<'_>,
    ) -> Result<Self, io::Error> {
        let state = AppState {
            sblex_service: Arc::new(sblex_service),
        };
        let router = app(state);
        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

        Ok(Self { router, listener })
    }

    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
    pub async fn run(self) -> std::io::Result<()> {
        tracing::info!(
            "Starting server at 'http://{}'",
            self.listener.local_addr()?
        );
        axum::serve(self.listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::warn!("signal received, starting graceful shutdown");
    // opentelemetry::global::shutdown_tracer_provider();
}
