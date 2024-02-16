use crate::db::queries::clients::Client;
use crate::redis::pool::RedisPooledConnectionType;
use bb8_redis::redis::AsyncCommands;

#[inline(always)]
pub async fn get_cached_client<'a>(
    redis_conn: &mut RedisPooledConnectionType<'a>,
    c_id: i16,
) -> Option<Client> {
    let cached_statement: Option<String> = redis_conn
        .get(format!("client:{}:statement", c_id))
        .await
        .unwrap();
    cached_statement.map(|s| serde_json::from_str(&s).unwrap())
}
