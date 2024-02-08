mod db;
mod routes;

use crate::db::connection::get_connection;
use crate::routes::statement::get_statement;
use crate::routes::transaction::do_transaction;
use ntex::web::{middleware, App, HttpServer};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "info");
    //env_logger::init();

    let server_port = std::env::var("RINHA_HTTP_PORT").unwrap_or("9999".to_string());

    let pool_size = std::env::var("RINHA_DB_POOL_SIZE")
        .unwrap_or("32".to_string())
        .parse::<u32>()
        .expect("Invalid RINHA_DB_POOL_SIZE");

    let pool = db::pool::create_db_pool(pool_size).await;

    let api_workers = std::env::var("RINHA_API_WORKERS")
        .unwrap_or("32".to_string())
        .parse::<usize>()
        .expect("Invalid RINHA_API_WORKERS");

    // Warm up the pool
    println!("Warming up the DB connection pool...");
    for _ in 0..pool_size {
        let _ = get_connection(&pool)
            .await
            .execute("SELECT 1", &[])
            .await
            .expect("Failed to execute test query");
    }

    println!(
        "Starting server on port={} / db_pool_size={} / api_workers={}",
        server_port, pool_size, api_workers
    );
    HttpServer::new(move || {
        let pool = pool.clone();
        App::new()
            .state(pool)
            .wrap(middleware::Logger::default())
            .service(do_transaction)
            .service(get_statement)
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .workers(api_workers)
    .run()
    .await
}
