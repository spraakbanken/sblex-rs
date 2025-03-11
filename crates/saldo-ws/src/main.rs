#![allow(unused)]
use axum::extract::Path;
use axum::{response::IntoResponse, routing::get, BoxError, Router};
use saldo_ws::startup;
use sblex_telemetry::telemetry;
use serde_json::json;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::from_filename(".env");
    env::set_var(
        "OTEL_SERVICE_NAME",
        env::var("SALDO_WS__OTEL_SERVICE_NAME")?,
    );
    let _guard = telemetry::init_telemetry()?;

    let app = saldo_ws::startup::app();
    // run it
    let address = &"0.0.0.0:3003".parse::<SocketAddr>()?;
    tracing::warn!("listening on {}", address);
    tracing::info!("try to call `curl -i http://127.0.0.1:3003/` (with trace)"); //Devskim: ignore DS137138
    tracing::info!("try to call `curl -i http://127.0.0.1:3003/health` (with NO trace)"); //Devskim: ignore DS137138
    let listener = tokio::net::TcpListener::bind(address).await?;
    startup::run(listener, app).await?;
    Ok(())
}
