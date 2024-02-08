use bb8::{Pool, PooledConnection};
use bb8_postgres::tokio_postgres::{Config, NoTls};
use bb8_postgres::PostgresConnectionManager;

use crate::config::RinhaConfig;

pub type DbPoolType = Pool<PostgresConnectionManager<NoTls>>;
pub type PooledConnectionType<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;

pub async fn create_db_pool(config: &RinhaConfig) -> DbPoolType {
    Pool::builder()
        .max_size(config.db.pool_size)
        .build(PostgresConnectionManager::new(
            Config::new()
                .user(&config.db.user)
                .password::<&str>(&config.db.password)
                .host(&config.db.host)
                .port(config.db.port)
                .dbname(&config.db.name)
                .to_owned(),
            NoTls,
        ))
        .await
        .expect("Failed to create database pool")
}
