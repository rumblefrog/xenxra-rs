use std::ops::{AddAssign, Deref, SubAssign};

use serde::Deserialize;

use crate::account::AccountID;

/// Guaranteed to be globally unique, and chronologically ordered.
/// Guaranteed to fit in u32.
#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TxID(u32);

impl From<u32> for TxID {
    fn from(id: u32) -> Self {
        TxID(id)
    }
}

/// Amount in cents.
/// Negative indicates a debit.
#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Amount(i64);

impl AddAssign for Amount {
    fn add_assign(&mut self, other: Amount) {
        self.0 += other.0;
    }
}

impl AddAssign<&Amount> for Amount {
    fn add_assign(&mut self, other: &Amount) {
        self.0 += other.0;
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, other: Amount) {
        self.0 -= other.0;
    }
}

impl SubAssign<&Amount> for Amount {
    fn sub_assign(&mut self, other: &Amount) {
        self.0 -= other.0;
    }
}

impl From<f64> for Amount {
    fn from(amount: f64) -> Self {
        Amount((amount * 100.0) as i64)
    }
}

impl From<Amount> for f64 {
    fn from(amount: Amount) -> Self {
        (amount.0 as f64) / 100.0
    }
}

impl From<&Amount> for f64 {
    fn from(amount: &Amount) -> Self {
        (amount.0 as f64) / 100.0
    }
}

impl From<i64> for Amount {
    fn from(amount: i64) -> Self {
        Amount(amount)
    }
}

impl Deref for Amount {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Amount> for String {
    fn from(amount: Amount) -> Self {
        format!("{:.4}", amount.0 as f64 / 100.0)
    }
}

#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

#[derive(PartialEq, Debug)]
pub enum DisputeStatus {
    Active,
    Resolved,
    Chargebacked,
}

#[derive(Debug)]
pub struct Transaction {
    pub typ: TransactionType,

    pub dispute_status: Option<DisputeStatus>,

    pub client_id: AccountID,

    pub amount: Amount,
}
