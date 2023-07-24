#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: String,
    pub database_url: String,
    pub api_key: String,
}

impl Config {
    pub fn build() -> Config {
        let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST must be set");
        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let api_key = std::env::var("API_KEY").expect("DATABASE_URL must be set");

        Config {
            server_host,
            server_port,
            database_url,
            api_key,
        }
    }
}
