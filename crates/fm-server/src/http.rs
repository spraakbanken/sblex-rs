use std::{io, net::SocketAddr, sync::Arc, time::Duration};

use axum::{routing, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use sblex_services::Morphology;
use tokio::{net::TcpListener, signal, sync::RwLock};
use tower_http::timeout::TimeoutLayer;

use self::handlers::get_morph::{get_saldo_morph, get_saldo_morph_w_cont};

mod handlers;

pub struct HttpServerConfig<'a> {
    pub port: u16,
    pub host: &'a str,
}

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
struct AppState<SM: Morphology> {
    saldo_morphology: Arc<RwLock<SM>>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: TcpListener,
}
impl HttpServer {
    pub async fn new(
        saldo_morphology: impl Morphology,
        config: HttpServerConfig<'_>,
    ) -> Result<Self, io::Error> {
        let state = AppState {
            saldo_morphology: Arc::new(RwLock::new(saldo_morphology)),
        };

        let router = create_router(state);
        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

        Ok(Self { router, listener })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
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

fn create_router<SM: Morphology>(state: AppState<SM>) -> Router {
    // let mut api = OpenApi::default();

    // let app = ApiRouter::new()
    //     .nest_api_service("/docs", docs_routes(state.clone()))
    //     .finish_api_with(&mut api, api_docs)
    //     .layer(Extension(Arc::new(api)))
    //     .with_state(state);

    Router::new()
        .route("/morph/{fragment}", routing::get(get_saldo_morph::<SM>))
        .route(
            "/morph-w-cont/{fragment}",
            routing::get(get_saldo_morph_w_cont::<SM>),
        )
        // .nest("/morph", morph_routes(state))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .with_state(state)
}
