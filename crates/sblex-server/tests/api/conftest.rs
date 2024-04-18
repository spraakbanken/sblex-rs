use tokio::net::TcpListener;

use sblex_server::startup;

pub struct Context {
    pub address: String,
}

pub async fn spawn_app() -> eyre::Result<Context> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr().unwrap().port();
    let address=  format!("http://127.0.0.1:{}",port);

    let app = startup::app();

    tokio::spawn(async move {
        startup::run(listener, app).await
    });
    Ok(Context {
        address
    })
}
