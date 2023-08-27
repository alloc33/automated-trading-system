use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            database_url: "postgres://market_app:@localhost:5432/market_db".to_string(),
            api_key: "hcYXTtlU67hjLdQ5LWbG8FG6qP2GDdEiBj8Oh+NMijs=".to_string(),
        }
    }
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
