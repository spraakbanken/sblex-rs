use std::net::{SocketAddr, TcpListener};

pub struct Context {
    pub address: String,
}

pub async fn spawn_app() -> Context {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(sblex_server::startup::app().into_make_service())
            .await
            .unwrap();
    });
    Context {
        address: format!("http://localhost:{}", addr.port()),
    }
}
