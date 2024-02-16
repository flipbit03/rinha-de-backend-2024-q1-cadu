use bb8_postgres::tokio_postgres::Row;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Incoming transaction request
#[derive(Deserialize)]
pub struct ClientTransactionRequest {
    pub valor: f64,
    pub tipo: String,
    pub descricao: Option<String>,
}

pub struct SanitizedClientTransactionRequest {
    pub valor: i64,
    pub tipo: String,
    pub tipo_multiplicador: i64,
    pub descricao: String,
}

const DEBIT: &str = "d";
const CREDIT: &str = "c";

impl TryFrom<&ClientTransactionRequest> for SanitizedClientTransactionRequest {
    type Error = ();

    fn try_from(value: &ClientTransactionRequest) -> Result<Self, Self::Error> {
        // Validate if valor is an integer
        if value.valor.trunc() != value.valor {
            return Err(());
        }

        // Valide a operação, e retorne 422 se for inválida
        let tipo_multiplicador = match value.tipo.as_str() {
            DEBIT => -1,
            CREDIT => 1,
            _ => {
                return Err(());
            }
        };

        // Valida a descrição, e já usa ela como retorno, se for válida
        // PS: Esse último bloco ficou terrível, mas eu economizei um .unwrap() :)
        match value.descricao {
            // Descricao nula? Whoops!
            None => Err(()),
            Some(ref desc) => {
                if desc.is_empty() || desc.len() > 10 {
                    return Err(());
                }

                Ok(Self {
                    valor: value.valor as i64,
                    tipo: value.tipo.clone(),
                    tipo_multiplicador,
                    descricao: desc.clone(),
                })
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClientStatementRow {
    pub valor: i64,
    pub tipo: String,
    pub descricao: Option<String>,
    pub realizada_em: DateTime<Utc>,
}

impl From<&Row> for ClientStatementRow {
    fn from(row: &Row) -> Self {
        Self {
            valor: row.get(0),
            tipo: row.get(1),
            descricao: row.get(2),
            realizada_em: row.get(3),
        }
    }
}
