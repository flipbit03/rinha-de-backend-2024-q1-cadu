use crate::cache::pool::{CachePoolType, CachePooledConnectionType};

#[inline(always)]
pub async fn get_cache_connection(pool: &CachePoolType) -> CachePooledConnectionType {
    pool.get().await.expect("Failed to get cache connection")
}
