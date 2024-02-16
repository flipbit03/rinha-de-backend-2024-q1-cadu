use crate::redis::pool::{RedisPoolType, RedisPooledConnectionType};

#[inline(always)]
pub async fn get_redis_connection(pool: &RedisPoolType) -> RedisPooledConnectionType {
    pool.get().await.expect("Failed to get redis connection")
}
