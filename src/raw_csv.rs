use serde::{Deserialize, Serialize};

use crate::account::AccountID;
use crate::transaction::TxID;

#[derive(Deserialize)]
pub enum TransactType {
    #[serde(rename = "deposit")]
    Deposit,

    #[serde(rename = "withdrawal")]
    Withdrawal,

    #[serde(rename = "dispute")]
    Dispute,

    #[serde(rename = "resolve")]
    Resolve,

    #[serde(rename = "chargeback")]
    Chargeback,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub r#type: TransactType,

    pub client: AccountID,

    pub tx: TxID,

    pub amount: Option<f64>,
}

#[derive(Serialize, Debug)]
pub struct Account {
    pub client: u16,

    pub available: f64,

    pub held: f64,

    pub total: f64,

    pub locked: bool,
}
