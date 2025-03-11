use saldo_ws::http::{HttpServer, HttpServerConfig};

pub struct Context {
    pub address: String,
}

pub async fn spawn_app() -> eyre::Result<Context> {
    let host = "127.0.0.1";
    let http_server_config = HttpServerConfig { port: 0, host };
    let http_server = HttpServer::new(http_server_config).await?;

    let port = http_server.local_addr()?.port();
    let address = format!("http://{}:{}", host, port);

    tokio::spawn(async move { http_server.run().await });
    Ok(Context { address })
}
