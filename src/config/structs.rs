use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub pool_size: u32,
}
