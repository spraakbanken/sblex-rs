use std::io;

use crate::http::handlers::lids;
use crate::http::handlers::system;

use axum::routing::get;
use axum::Router;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;

mod handlers;

pub fn app() -> Router {
    Router::new()
        .route("/lid/json/{lid}", get(lids::lookup_lid_json)) // request processed inside span
        .route("/lid/xml/{lid}", get(lids::lookup_lid_xml)) // request processed inside span
        .route("/lid/html/{lid}", get(lids::lookup_lid_html)) // request processed inside span
        // include trace context as header into the response
        .layer(OtelInResponseLayer)
        // opentelemetry_tracing_layer setup `TraceLayer`,
        // that is provided by tower-http so you have to add that as a dependency.
        .layer(OtelAxumLayer::default())
        .route("/health", get(system::health))
        .route("/version/json", get(system::version))
}

use tokio::{net::TcpListener, signal};
pub struct HttpServer {
    router: axum::Router,
    listener: TcpListener,
}

pub struct HttpServerConfig<'a> {
    pub port: u16,
    pub host: &'a str,
}
impl HttpServer {
    pub async fn new(config: HttpServerConfig<'_>) -> Result<Self, io::Error> {
        let router = app();
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
    opentelemetry::global::shutdown_tracer_provider();
}
