use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

use crate::account::{Account, AccountID};
use crate::raw_csv;
use crate::transaction::{DisputeStatus, Transaction, TransactionType, TxID};

#[derive(Default, Debug)]
pub struct Ledger {
    accounts: HashMap<AccountID, Account>,

    transactions: HashMap<TxID, Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn process(&mut self, transaction: raw_csv::Transaction) {
        self.accounts.entry(transaction.client.clone()).or_default();

        match transaction.r#type {
            raw_csv::TransactType::Dispute
            | raw_csv::TransactType::Resolve
            | raw_csv::TransactType::Chargeback => {
                self.dispute(&transaction.tx, &transaction.client, transaction.r#type)
            }
            raw_csv::TransactType::Deposit => self.deposit(&transaction),
            raw_csv::TransactType::Withdrawal => self.withdraw(&transaction),
        }
    }

    #[inline]
    pub fn deposit(&mut self, tx: &raw_csv::Transaction) {
        let amount = tx.amount.unwrap().into();

        self.accounts.get_mut(&tx.client).unwrap().deposit(&amount);

        self.transactions.insert(
            tx.tx.clone(),
            Transaction {
                typ: TransactionType::Deposit,
                dispute_status: None,
                client_id: tx.client.clone(),
                amount,
            },
        );
    }

    #[inline]
    pub fn withdraw(&mut self, tx: &raw_csv::Transaction) {
        let amount = tx.amount.unwrap().into();

        self.accounts.get_mut(&tx.client).unwrap().withdraw(&amount);

        self.transactions.insert(
            tx.tx.clone(),
            Transaction {
                typ: TransactionType::Withdrawal,
                dispute_status: None,
                client_id: tx.client.clone(),
                amount,
            },
        );
    }

    pub fn dispute(&mut self, tx_id: &TxID, client_id: &AccountID, status: raw_csv::TransactType) {
        if let (Some(acc), Some(tx)) = (
            self.accounts.get_mut(client_id),
            self.transactions.get_mut(tx_id),
        ) {
            let amount = &tx.amount;

            match status {
                raw_csv::TransactType::Dispute => {
                    // Possible that available balance can go into negative
                    acc.available_balance_mut().sub_assign(amount);
                    acc.held_balance_mut().add_assign(amount);

                    tx.dispute_status = Some(DisputeStatus::Active);
                }

                // if the tx isn't under dispute,
                // you can ignore the resolve and assume this is an error on our partner's side
                raw_csv::TransactType::Resolve
                    if tx.dispute_status == Some(DisputeStatus::Active) =>
                {
                    acc.available_balance_mut().add_assign(amount);
                    acc.held_balance_mut().sub_assign(amount);

                    tx.dispute_status = Some(DisputeStatus::Resolved);
                }

                // if the tx specified doesn't exist, or the tx isn't under dispute,
                // you can ignore chargeback and assume this is an error on our partner's side
                raw_csv::TransactType::Chargeback
                    if tx.dispute_status == Some(DisputeStatus::Active) =>
                {
                    acc.held_balance_mut().sub_assign(amount);

                    acc.freeze();

                    tx.dispute_status = Some(DisputeStatus::Chargebacked);
                }

                _ => {}
            }
        }
    }

    pub fn snapshot(&self) -> Vec<raw_csv::Account> {
        self.accounts
            .iter()
            .map(|(id, acc)| raw_csv::Account {
                client: id.into(),
                available: acc.available_balance().into(),
                held: acc.held_balance().into(),
                total: acc.total_balance().into(),
                locked: acc.is_frozen(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_csv::{TransactType, Transaction};

    #[test]
    fn test_ledger() {
        let txs = [
            Transaction {
                r#type: TransactType::Deposit,
                client: 1.into(),
                tx: 1.into(),
                amount: 1.0.into(),
            },
            Transaction {
                r#type: TransactType::Deposit,
                client: 2.into(),
                tx: 2.into(),
                amount: 2.0.into(),
            },
            Transaction {
                r#type: TransactType::Deposit,
                client: 1.into(),
                tx: 3.into(),
                amount: 2.0.into(),
            },
            Transaction {
                r#type: TransactType::Withdrawal,
                client: 1.into(),
                tx: 4.into(),
                amount: 1.5.into(),
            },
            Transaction {
                r#type: TransactType::Withdrawal,
                client: 2.into(),
                tx: 5.into(),
                amount: 3.0.into(),
            },
        ];

        let mut ledger = super::Ledger::new();

        for tx in txs {
            ledger.process(tx);
        }

        let snapshot = ledger.snapshot();

        assert_eq!(snapshot.len(), 2);

        let client_1 = snapshot.iter().find(|c| c.client == 1);
        let client_2 = snapshot.iter().find(|c| c.client == 2);

        assert!(client_1.is_some());
        assert!(client_2.is_some());

        assert_eq!(client_1.unwrap().available, 1.5);
        assert_eq!(client_1.unwrap().held, 0.0);
        assert_eq!(client_1.unwrap().total, 1.5);
        assert_eq!(client_1.unwrap().locked, false);

        assert_eq!(client_2.unwrap().available, 2.0);
        assert_eq!(client_2.unwrap().held, 0.0);
        assert_eq!(client_2.unwrap().total, 2.0);
        assert_eq!(client_2.unwrap().locked, false);
    }
}
