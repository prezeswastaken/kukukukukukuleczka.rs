use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use openai_api_rust::chat::*;
use openai_api_rust::completions::*;
use openai_api_rust::*;

use kukuleczka_backend::{config::Config, cv_builder::CVBuilder, forms::BasicForm};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();
    let config = Config::new();
    let auth = Auth::new(config.get_openai_api_key());
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: None,
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: "Hello!".to_string(),
        }],
    };
    // let response = openai.chat_completion_create(&body);
    // let choice = response.unwrap().choices;
    // let message = &choice[0].message.as_ref().unwrap();
    // println!("Response: {}", message.content);

    let basic_form = BasicForm {
        full_name: "Michael Scott".to_string(),
        email: "test@test.com".to_string(),
        programming_languages: vec!["PHP".to_string(), "Rust".to_string()],
        education_level: "Inżynier".to_string(),
    };

    let cv_string = CVBuilder::from(basic_form).create_cv_string();
    match cv_string {
        Ok(cv) => println!("CV: {}", cv),
        Err(e) => eprintln!("Error: {}", e),
    }

    let app = Router::new().route("/", get(hello));

    let app_port = config.get_app_port();
    println!("Listening on port {} 󱓟", app_port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{app_port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!"
}
