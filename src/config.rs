use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub http_port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize)]
pub struct RinhaConfig {
    pub db: DbConfig,
    pub api: ApiConfig,
}

impl RinhaConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = Config::builder()
            .add_source(Environment::with_prefix("RINHA").separator("__"))
            .build()
            .unwrap();

        s.try_deserialize()
    }
}
