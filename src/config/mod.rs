pub mod structs;

use config::{Config, Environment};
use serde::Deserialize;
use structs::{DbConfig, HttpConfig, RedisConfig};

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub http: HttpConfig,
    pub db: DbConfig,
    pub redis: RedisConfig,
    pub workers: usize,
}

#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    pub db: DbConfig,
    pub redis: RedisConfig,
}

pub trait GetFromEnv {
    fn get_from_env() -> Self
    where
        Self: Sized;
}

impl<'a, T> GetFromEnv for T
where
    T: Deserialize<'a>,
{
    fn get_from_env() -> Self {
        Config::builder()
            .add_source(Environment::with_prefix("RINHA").separator("__"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
