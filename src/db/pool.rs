use crate::config::structs::DbConfig;
use crate::db::connection::get_db_connection;
use bb8::{Pool, PooledConnection};
use bb8_postgres::tokio_postgres::{Config, NoTls};
use bb8_postgres::PostgresConnectionManager;

pub type DbPoolType = Pool<PostgresConnectionManager<NoTls>>;
pub type DbPooledConnectionType<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;

pub async fn create_db_pool(db: &DbConfig) -> DbPoolType {
    let pool = Pool::builder()
        .max_size(db.pool_size)
        .build(PostgresConnectionManager::new(
            Config::new()
                .user(&db.user)
                .password(&db.password)
                .host(&db.host)
                .port(db.port)
                .dbname(&db.name)
                .to_owned(),
            NoTls,
        ))
        .await
        .expect("Failed to create database pool");

    // Warm up the db pool
    println!("Warming up the DB connection pool...");
    for _ in 0..db.pool_size {
        let _ = get_db_connection(&pool)
            .await
            .execute("SELECT 1", &[])
            .await
            .expect("Failed to execute test query");
    }

    pool
}
