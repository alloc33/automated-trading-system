use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub api_key: String,
}

impl AppConfig {
    pub fn build() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let api_key = env::var("API_KEY").expect("DATABASE_URL must be set");

        AppConfig {
            database_url,
            api_key,
        }
    }
}
