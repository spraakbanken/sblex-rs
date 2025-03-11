use crate::routes::lids;
use crate::routes::system;

use axum::routing::get;
use axum::Router;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;

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

pub async fn run(listener: TcpListener, app: Router) -> std::io::Result<()> {
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
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
