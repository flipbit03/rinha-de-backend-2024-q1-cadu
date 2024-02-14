use bb8::{Pool, PooledConnection};
use bb8_postgres::tokio_postgres::{Config, NoTls};
use bb8_postgres::PostgresConnectionManager;

pub type DbPoolType = Pool<PostgresConnectionManager<NoTls>>;
pub type DbPooledConnectionType<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;

pub async fn create_db_pool(pool_max_size: u32) -> DbPoolType {
    let db_user = std::env::var("RINHA_DB_USER").expect("RINHA_DB_USER must be set");
    let db_password = std::env::var("RINHA_DB_PASSWORD").expect("RINHA_DB_PASSWORD must be set");
    let db_host = std::env::var("RINHA_DB_HOST").expect("RINHA_DB_HOST must be set");
    let db_port = std::env::var("RINHA_DB_PORT")
        .expect("RINHA_DB_PORT must be set")
        .parse::<u16>()
        .expect("RINHA_DB_PORT must be a valid port number");
    let db_name = std::env::var("RINHA_DB_NAME").expect("RINHA_DB_NAME must be set");

    Pool::builder()
        .max_size(pool_max_size)
        .build(PostgresConnectionManager::new(
            Config::new()
                .user(&db_user)
                .password(&db_password)
                .host(&db_host)
                .port(db_port)
                .dbname(&db_name)
                .to_owned(),
            NoTls,
        ))
        .await
        .expect("Failed to create database pool")
}
