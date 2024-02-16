use crate::db::pool::DbPooledConnectionType;
use bb8_postgres::tokio_postgres::{Row, Transaction};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub id: i16,
    pub limit: i64,
    pub balance: i64,
}

impl From<&Row> for Client {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            limit: row.get(1),
            balance: row.get(2),
        }
    }
}
static GET_CLIENT_BY_ID_QUERY: &str = "SELECT i,l,b from c where i = $1";

#[inline(always)]
pub async fn get_client_by_id<'a>(
    db: &DbPooledConnectionType<'a>,
    client_id: i16,
) -> Option<Client> {
    let client_row = &db
        .query_opt(GET_CLIENT_BY_ID_QUERY, &[&client_id])
        .await
        .unwrap();

    client_row.as_ref().map(Client::from)
}

static GET_CLIENT_FOR_UPDATE_BY_ID_QUERY: &str = "SELECT i,l,b from c where i = $1 FOR UPDATE";

/// Gets a
#[inline(always)]
pub async fn get_client_for_update_by_id<'a>(
    db: &Transaction<'a>,
    client_id: i16,
) -> Option<Client> {
    let client_row = &db
        .query_opt(GET_CLIENT_FOR_UPDATE_BY_ID_QUERY, &[&client_id])
        .await
        .unwrap();

    client_row.as_ref().map(Client::from)
}

//
// Update Client balance by ID
//

const UPDATE_CLIENT_BALANCE_BY_ID_QUERY: &str = "UPDATE c SET b = $1 WHERE i = $2";

#[inline(always)]
pub async fn update_client_balance_by_id<'a>(
    db: &Transaction<'a>,
    client_id: i16,
    new_balance: i64,
) {
    let _ = db
        .execute(
            UPDATE_CLIENT_BALANCE_BY_ID_QUERY,
            &[&new_balance, &client_id],
        )
        .await
        .unwrap();
}
