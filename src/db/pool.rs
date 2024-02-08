use bb8::{Pool, PooledConnection};
use bb8_postgres::tokio_postgres::{Config, NoTls};
use bb8_postgres::PostgresConnectionManager;

pub type DbPoolType = Pool<PostgresConnectionManager<NoTls>>;
pub type PooledConnectionType<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;

pub async fn create_db_pool(pool_max_size: u32) -> DbPoolType {
    Pool::builder()
        .max_size(pool_max_size)
        .build(PostgresConnectionManager::new(
            Config::new()
                .user(
                    std::env::var("RINHA_DB_USER")
                        .expect("RINHA_DB_USER must be set")
                        .as_ref(),
                )
                .password::<String>(
                    std::env::var("RINHA_DB_PASSWORD").expect("RINHA_DB_PASSWORD must be set"),
                )
                .host(
                    std::env::var("RINHA_DB_HOST")
                        .expect("RINHA_DB_HOST must be set")
                        .as_ref(),
                )
                .port(
                    std::env::var("RINHA_DB_PORT")
                        .expect("RINHA_DB_PORT must be set")
                        .parse::<u16>()
                        .expect("RINHA_DB_PORT must be a valid port number"),
                )
                .dbname(
                    std::env::var("RINHA_DB_NAME")
                        .expect("RINHA_DB_NAME must be set")
                        .as_ref(),
                )
                .to_owned(),
            NoTls,
        ))
        .await
        .expect("Failed to create database pool")
}
