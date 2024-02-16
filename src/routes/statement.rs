use crate::db::connection::get_db_connection;
use crate::db::pool::DbPoolType;
use crate::db::queries::statement::get_client_statement;
use crate::redis::connection::get_redis_connection;
use crate::redis::pool::RedisPoolType;
use bb8_redis::redis::AsyncCommands;
use ntex::http::StatusCode;
use ntex::web;
use ntex::web::{get, HttpResponse};

#[get("/clientes/{c_id}/extrato")]
pub async fn get_statement(
    c_id: web::types::Path<i16>,
    db_pool: web::types::State<DbPoolType>,
    redis_pool: web::types::State<RedisPoolType>,
) -> HttpResponse {
    let mut redis_conn = get_redis_connection(&redis_pool).await;

    let cached_statement: Option<String> = redis_conn
        .get(format!("client:{}:statement", c_id))
        .await
        .unwrap();

    // If the statement is cached, return it
    if let Some(s) = cached_statement {
        println!("Statement found in cache");
        return HttpResponse::Ok().content_type("application/json").body(s);
    };

    let db_conn = get_db_connection(&db_pool).await;

    match get_client_statement(&db_conn, *c_id).await {
        None => HttpResponse::new(StatusCode::NOT_FOUND),
        Some(statement) => {
            let serialized = serde_json::to_string(&statement).unwrap();
            let _: String = redis_conn
                .set(format!("client:{}:statement", c_id), serialized)
                .await
                .unwrap();
            HttpResponse::Ok().json(&statement)
        }
    }
}
