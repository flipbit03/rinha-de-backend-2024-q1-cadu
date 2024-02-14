use crate::db::pool::{DbPoolType, DbPooledConnectionType};

#[inline(always)]
pub async fn get_db_connection(pool: &DbPoolType) -> DbPooledConnectionType {
    pool.get().await.expect("Failed to get database connection")
}
