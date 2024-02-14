use crate::db::connection::get_db_connection;
use crate::db::pool::DbPoolType;
use crate::db::queries::clients::{get_client_for_update_by_id, update_client_balance_by_id};
use crate::db::queries::transaction::insert_new_client_transaction;
use crate::db::queries::transaction::structs::{
    ClientTransactionRequest, SanitizedClientTransactionRequest,
};
use log::error;
use ntex::http::StatusCode;
use ntex::web;
use ntex::web::{post, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct TransactionResponse {
    pub limite: i64,
    pub saldo: i64,
}

#[post("/clientes/{client_id}/transacoes")]
pub async fn do_transaction(
    client_id: web::types::Path<i16>,
    pool: web::types::State<DbPoolType>,
    transaction_request: web::types::Json<ClientTransactionRequest>,
) -> HttpResponse {
    // Valide a entrada ou já morra com 422
    let sanitized_request = match SanitizedClientTransactionRequest::try_from(&*transaction_request)
    {
        Ok(sanitized) => sanitized,
        Err(_) => {
            // Invalid request (return 422)
            return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
        }
    };

    let mut db_conn = get_db_connection(&pool).await;
    let db = db_conn.transaction().await.unwrap();

    // get client from Database
    let client = get_client_for_update_by_id(&db, *client_id).await;

    match client {
        // Client not found (return 404)
        None => HttpResponse::new(StatusCode::NOT_FOUND),
        Some(client) => {
            // Calculate new balance
            let new_balance =
                client.balance + (sanitized_request.valor * sanitized_request.tipo_multiplicador);

            // Check if new balance would surpass the client's credit limit
            // return 422 if it would be (don't update the balance)
            if new_balance < -client.limit {
                return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
            }

            // Update client balance
            update_client_balance_by_id(&db, *client_id, new_balance).await;

            // Insert new transaction into the database
            if let Err(e) = insert_new_client_transaction(&db, *client_id, &sanitized_request).await
            {
                error!("Failed to insert transaction: {:?}", e);
                // Failed to insert transaction (return 422)
                return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
            }

            match db.commit().await {
                Ok(_) => HttpResponse::Ok().json(&TransactionResponse {
                    limite: client.limit,
                    saldo: new_balance,
                }),
                Err(e) => {
                    eprintln!("Failed to commit transaction: {:?}", e);
                    // Failed to commit transaction (return 500)
                    return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
                }
            }
        }
    }
}
