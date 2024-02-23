use crate::db::pool::{DbPoolType, PooledConnectionType};

#[inline(always)]
pub async fn get_connection(pool: &DbPoolType) -> PooledConnectionType {
    match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}
