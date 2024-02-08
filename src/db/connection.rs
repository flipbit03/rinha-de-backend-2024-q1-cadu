use crate::db::pool::{DbPoolType, PooledConnectionType};

#[inline(always)]
pub async fn get_connection(pool: &DbPoolType) -> PooledConnectionType {
    pool.get().await.unwrap()
}
