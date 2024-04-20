use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use kukuleczka_backend::config::Config;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    let config = Config::new();
    let app_port = config.get_app_port();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{app_port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!"
}
