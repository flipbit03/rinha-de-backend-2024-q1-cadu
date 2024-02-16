use crate::config::structs::RedisConfig;
use crate::redis::connection::get_redis_connection;
use bb8::{Pool, PooledConnection};
use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;

pub type RedisPoolType = Pool<RedisConnectionManager>;
pub type RedisPooledConnectionType<'a> = PooledConnection<'a, RedisConnectionManager>;

pub async fn create_redis_pool(redis: &RedisConfig) -> RedisPoolType {
    let redis_connection_string = format!("redis://{}:{}/{}", &redis.host, &redis.port, &redis.db);
    let pool = Pool::builder()
        .max_size(redis.pool_size)
        .build(
            RedisConnectionManager::new(redis_connection_string)
                .expect("Failed to create Redis connection manager"),
        )
        .await
        .expect("Failed to create Redis pool");

    // Warm up the redis pool
    println!("Warming up the Redis connection pool...");
    for i in 0..redis.pool_size {
        let mut conn = get_redis_connection(&pool).await;
        let _: String = conn.set("test", i).await.expect("Failed to set test key");
        let _: u32 = conn.del("test").await.expect("Failed to delete test key");
    }

    pool
}
