pub struct Config {
    openai_api_key: String,
    app_port: String,
    storage_path: String,
    app_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file");
        let app_port = std::env::var("APP_PORT").expect("APP_PORT is not set");
        println!("APP_PORT: {}", app_port);
        let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
        let storage_path =
            std::env::var("STORAGE_ABSOLUTE_PATH").expect("STORAGE_ABSOLUTE_PATH is not set");
        let app_url = std::env::var("APP_URL").expect("APP_URL is not set");

        Self {
            openai_api_key,
            app_port,
            storage_path,
            app_url,
        }
    }

    pub fn get_openai_api_key(&self) -> &str {
        &self.openai_api_key
    }

    pub fn get_app_port(&self) -> &str {
        &self.app_port
    }

    pub fn get_storage_path(&self) -> &str {
        &self.storage_path
    }

    pub fn get_app_url(&self) -> &str {
        &self.app_url
    }
}
