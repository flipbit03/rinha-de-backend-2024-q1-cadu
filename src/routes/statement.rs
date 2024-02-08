use crate::db::connection::get_connection;
use crate::db::pool::DbPoolType;
use crate::db::queries::clients::get_client_by_id;
use crate::db::queries::transaction::get_last_10_transactions;
use crate::db::queries::transaction::structs::ClientStatementRow;
use chrono::{DateTime, Utc};
use ntex::http::StatusCode;
use ntex::web;
use ntex::web::{get, HttpResponse};
use serde::Serialize;
use std::time::SystemTime;

/// Client Statement Header
/// Containing the client's current balance and credit limit,
/// together with the statement date
#[derive(Serialize)]
struct ClientStatementHeader {
    pub total: i64,
    pub data_extrato: DateTime<Utc>,
    pub limite: i64,
}

/// Client Statement
#[derive(Serialize)]
struct ClientStatement {
    pub saldo: ClientStatementHeader,
    pub ultimas_transacoes: Vec<ClientStatementRow>,
}

#[get("/clientes/{c_id}/extrato")]
pub async fn get_statement(
    c_id: web::types::Path<i16>,
    pool: web::types::State<DbPoolType>,
) -> HttpResponse {
    let conn = get_connection(&pool).await;

    match get_client_by_id(&conn, *c_id).await {
        None => HttpResponse::new(StatusCode::NOT_FOUND),
        Some(client) => HttpResponse::Ok().json(&ClientStatement {
            saldo: ClientStatementHeader {
                total: client.balance,
                data_extrato: SystemTime::now().into(),
                limite: client.limit,
            },
            ultimas_transacoes: get_last_10_transactions(&conn, *c_id).await,
        }),
    }
}
