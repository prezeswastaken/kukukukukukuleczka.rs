pub struct Config {
    openai_api_key: String,
    app_port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file");
        let app_port = std::env::var("APP_PORT").expect("APP_PORT is not set");
        println!("APP_PORT: {}", app_port);
        let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
        println!("OPENAI_API_KEY: {}", openai_api_key);

        Self {
            openai_api_key,
            app_port,
        }
    }

    pub fn get_openai_api_key(&self) -> &str {
        &self.openai_api_key
    }

    pub fn get_app_port(&self) -> &str {
        &self.app_port
    }
}
