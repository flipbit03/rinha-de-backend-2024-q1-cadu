mod cache;
mod db;
mod routes;

use crate::cache::connection::get_cache_connection;
use crate::cache::pool::create_redis_pool;
use crate::db::connection::get_db_connection;
use crate::db::pool::create_db_pool;
use crate::routes::statement::get_statement;
use crate::routes::transaction::do_transaction;
use bb8_redis::redis::AsyncCommands;
use ntex::web::{middleware, App, HttpServer};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "info");
    //env_logger::init();

    let server_port = std::env::var("RINHA_HTTP_PORT").unwrap_or("9999".to_string());

    let api_workers = std::env::var("RINHA_API_WORKERS")
        .unwrap_or("32".to_string())
        .parse::<usize>()
        .expect("Invalid RINHA_API_WORKERS");

    //
    // Create a (warmed up) pool of database connections
    //
    let db_pool_size = std::env::var("RINHA_DB_POOL_SIZE")
        .unwrap_or("32".to_string())
        .parse::<u32>()
        .expect("Invalid RINHA_DB_POOL_SIZE");
    let db_pool = create_db_pool(db_pool_size).await;

    // Warm up the db pool
    println!("Warming up the DB connection pool...");
    for _ in 0..db_pool_size {
        let _ = get_db_connection(&db_pool)
            .await
            .execute("SELECT 1", &[])
            .await
            .expect("Failed to execute test query");
    }

    //
    // Create a (warmed up) pool of Redis connections
    //
    let cache_pool_size = std::env::var("RINHA_CACHE_POOL_SIZE")
        .unwrap_or("32".to_string())
        .parse::<u32>()
        .expect("Invalid RINHA_CACHE_POOL_SIZE");
    let cache_pool = create_redis_pool(cache_pool_size).await;

    // Warm up the cache pool
    println!("Warming up the Redis connection pool...");
    for i in 0..cache_pool_size {
        let mut conn = get_cache_connection(&cache_pool).await;
        let _: String = conn.set("test", i).await.expect("Failed to set test key");
        let _: u32 = conn.del("test").await.expect("Failed to delete test key");
    }

    println!(
        "Starting server on port={} / db_pool_size={} / cache_pool_size={} / api_workers={}",
        server_port, db_pool_size, cache_pool_size, api_workers
    );
    HttpServer::new(move || {
        let cloned_db_pool = db_pool.clone();
        let cloned_cache_pool = cache_pool.clone();
        App::new()
            .state(cloned_db_pool)
            .state(cloned_cache_pool)
            .wrap(middleware::Logger::default())
            .service(do_transaction)
            .service(get_statement)
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .workers(api_workers)
    .run()
    .await
}
