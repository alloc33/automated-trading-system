use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub api_key: String,
    pub trade_signal_max_retries: u8,
    pub trade_signal_retry_delay: f64,
}

impl Config {
    pub fn build() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let api_key = env::var("API_KEY").expect("DATABASE_URL must be set");
        let trade_signal_max_retries = env::var("TRADE_SIGNAL_MAX_RETRIES")
            .expect("TRADE_SIGNAL_MAX_RETRIES must be set")
            .parse::<u8>()
            .expect("TRADE_SIGNAL_MAX_RETRIES must be a number");
        let trade_signal_retry_delay = env::var("TRADE_SIGNAL_RETRY_DELAY")
            .expect("TRADE_SIGNAL_RETRY_DELAY must be set")
            .parse::<f64>()
            .expect("TRADE_SIGNAL_RETRY_DELAY must be a number");

        Config {
            database_url,
            api_key,
            trade_signal_max_retries,
            trade_signal_retry_delay,
        }
    }
}
