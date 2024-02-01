use fm_server::{server, startup, state::AppState};

#[tokio::main]
async fn main() {
    let state = AppState::from_path("assets/saldo.lex").unwrap();

    let app = server::create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    startup::run(listener, app).await.unwrap();
}
