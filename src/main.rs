mod config;
mod db;
mod routes;

use crate::routes::statement::get_statement;
use crate::routes::transaction::do_transaction;
use crate::{config::RinhaConfig, db::connection::get_connection};
use log::error;
use ntex::web::{middleware, App, HttpServer};

#[ntex::main]
async fn main() -> std::io::Result<()> {

    // Logging is only enabled in debug mode
    #[cfg(debug_assertions)]
    {
        println!("Enabling debug logging...");
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();
    }

    let config = match RinhaConfig::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let pool = db::pool::create_db_pool(&config).await;

    // Warm up the pool
    println!("Warming up the DB connection pool...");
    for _ in 0..config.db.pool_size {
        let _ = get_connection(&pool)
            .await
            .execute("SELECT 1", &[])
            .await
            .expect("Failed to execute test query");
    }

    println!(
        "Starting server on port={} / db_pool_size={} / api_workers={}",
        config.api.http_port, config.db.pool_size, config.api.workers
    );
    HttpServer::new(move || {
        let pool = pool.clone();
        App::new()
            .state(pool)
            .wrap(middleware::Logger::default())
            .service(do_transaction)
            .service(get_statement)
    })
    .bind(format!("0.0.0.0:{}", config.api.http_port))?
    .workers(config.api.workers)
    .run()
    .await
}
