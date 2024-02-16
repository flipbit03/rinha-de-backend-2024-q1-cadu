use crate::db::pool::DbPooledConnectionType;
use crate::db::queries::clients::get_client_by_id;
use crate::db::queries::transaction::get_last_10_transactions;
use crate::db::queries::transaction::structs::ClientStatementRow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Client Statement Header
/// Containing the client's current balance and credit limit,
/// together with the statement date
#[derive(Deserialize, Serialize)]
pub struct ClientStatementHeader {
    pub total: i64,
    pub data_extrato: DateTime<Utc>,
    pub limite: i64,
}

/// Client Statement
#[derive(Deserialize, Serialize)]
pub struct ClientStatement {
    pub saldo: ClientStatementHeader,
    pub ultimas_transacoes: Vec<ClientStatementRow>,
}

pub async fn get_client_statement<'a>(
    db_conn: &DbPooledConnectionType<'a>,
    c_id: i16,
) -> Option<ClientStatement> {
    match get_client_by_id(db_conn, c_id).await {
        None => None,
        Some(client) => Some(ClientStatement {
            saldo: ClientStatementHeader {
                total: client.balance,
                data_extrato: SystemTime::now().into(),
                limite: client.limit,
            },
            ultimas_transacoes: get_last_10_transactions(db_conn, c_id).await,
        }),
    }
}
