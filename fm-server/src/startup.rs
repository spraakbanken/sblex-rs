use axum::Router;
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener, app: Router) -> std::io::Result<()> {
    axum::serve(listener, app).await
}
