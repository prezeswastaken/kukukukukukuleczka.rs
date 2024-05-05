use std::sync::Arc;

use axum::routing::post;
use axum::{response::IntoResponse, routing::get, Json, Router};
use kukuleczka_backend::enums::{Language, Technology};
use kukuleczka_backend::handlers::*;
use kukuleczka_backend::jobs::{get_jobs, JobCheckRequest};
use serde_json::json;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use kukuleczka_backend::{
    config::Config,
    handlers::{basic_create, pdf},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TEST
    // let jobs = get_jobs();
    // let job = jobs.get(0).unwrap();
    // let job_request = JobCheckRequest {
    //     years_of_experience: 5,
    //     technologies: vec![  Technology::Java, Technology::Go, Technology::JavaScript, Technology::TypeScript ],
    //     languages: vec![Language::Polski],
    //     soft_skiills: 0,
    // };
    // let score = job.get_score(&job_request);
    // println!("Score: {:?}", score.unwrap());

    // END OF TEST

    let config = Arc::new(RwLock::new(Config::new()));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/", get(hello))
        .route("/basic/create", post(basic_create))
        .route("/pdf", post(pdf))
        .route("/jobs", get(show_jobs))
        .route("/check-job", post(check_job))
        .layer(cors)
        .with_state(Arc::clone(&config))
        .nest_service("/storage", ServeDir::new("storage"));

    let config_read = config.read().await;
    let app_port = config_read.get_app_port();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{app_port}"))
        .await
        .expect("Couldn't create listener");
    axum::serve(listener, app)
        .await
        .expect("Couldn't start server");
    println!("Listening on port {} 󱓟", app_port);

    Ok(())
}

async fn hello() -> impl IntoResponse {
    Json(json!({"message": "Hello, from github action improved!"}))
}
