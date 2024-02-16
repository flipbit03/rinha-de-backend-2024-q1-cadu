use ntex::web::{middleware, App, HttpServer};
use rinha_de_backend_2024_q1_cadu::config::{ApiConfig, GetFromEnv};
use rinha_de_backend_2024_q1_cadu::db::pool::create_db_pool;
use rinha_de_backend_2024_q1_cadu::redis::pool::create_redis_pool;
use rinha_de_backend_2024_q1_cadu::routes::statement::get_statement;
use rinha_de_backend_2024_q1_cadu::routes::transaction::do_transaction;
use std::io::Error;

#[ntex::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Load the API configuration from environment variables
    let api_config = ApiConfig::get_from_env();

    // Create a (warmed up) pool of database connections
    let db_pool = create_db_pool(&api_config.db).await;

    // Create a (warmed up) pool of Redis connections
    let cache_pool = create_redis_pool(&api_config.redis).await;

    println!(
        "Starting server on port={} / db_pool_size={} / cache_pool_size={} / api_workers={}",
        &api_config.http.port,
        &api_config.db.pool_size,
        &api_config.redis.pool_size,
        &api_config.workers
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
    .bind(format!("0.0.0.0:{}", &api_config.http.port))?
    .workers(api_config.workers)
    .run()
    .await
}
