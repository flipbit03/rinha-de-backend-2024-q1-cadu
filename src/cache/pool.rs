use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;

pub type CachePoolType = Pool<RedisConnectionManager>;
pub type CachePooledConnectionType<'a> = PooledConnection<'a, RedisConnectionManager>;

pub async fn create_redis_pool(pool_max_size: u32) -> CachePoolType {
    let redis_host = std::env::var("RINHA_REDIS_HOST").expect("RINHA_REDIS_HOST must be set");
    let redis_port = std::env::var("RINHA_REDIS_PORT")
        .expect("RINHA_REDIS_PORT must be set")
        .parse::<u16>()
        .expect("RINHA_REDIS_PORT must be a valid port number in the range 1-65535");
    let redis_db = std::env::var("RINHA_REDIS_DB")
        .expect("RINHA_REDIS_DB must be set")
        .parse::<u8>()
        .expect("RINHA_REDIS_DB must be a valid database number in the range 0-255");

    let redis_url = format!("redis://{}:{}/{}", redis_host, redis_port, redis_db);

    Pool::builder()
        .max_size(pool_max_size)
        .build(
            RedisConnectionManager::new(redis_url)
                .expect("Failed to create Redis connection manager"),
        )
        .await
        .expect("Failed to create Redis pool")
}
