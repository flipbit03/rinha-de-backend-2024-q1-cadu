pub mod structs;

use crate::db::pool::PooledConnectionType;
use crate::db::queries::transaction::structs::{
    ClientStatementRow, SanitizedClientTransactionRequest,
};
use bb8_postgres::tokio_postgres::{Error, Transaction};

const INSERT_NEW_TRANSACTION_QUERY: &str = "INSERT INTO t (c, o, a, d) VALUES ($1, $2, $3, $4)";

/// Insert incoming transaction
#[inline(always)]
pub async fn insert_new_client_transaction<'a>(
    db: &Transaction<'a>,
    client_id: i16,
    transaction: &SanitizedClientTransactionRequest,
) -> Result<u64, Error> {
    db.execute(
        INSERT_NEW_TRANSACTION_QUERY,
        &[
            &client_id,
            &transaction.tipo,
            &transaction.valor,
            &transaction.descricao,
        ],
    )
    .await
}

const GET_LAST_10_TRANSACTIONS: &str =
    "SELECT a, o, d, t FROM t WHERE c = $1 ORDER BY t DESC LIMIT 10";

/// Get last N transactions for a client
#[inline(always)]
pub async fn get_last_10_transactions<'a>(
    db: &PooledConnectionType<'a>,
    client_id: i16,
) -> Vec<ClientStatementRow> {
    let rows = &db
        .query(GET_LAST_10_TRANSACTIONS, &[&client_id])
        .await
        .unwrap();

    rows.iter().map(ClientStatementRow::from).collect()
}
