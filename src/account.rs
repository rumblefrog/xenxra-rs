use serde::Deserialize;

use crate::transaction::Amount;

/// Guaranteed to fit within u16.
#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct AccountID(u16);

impl From<u16> for AccountID {
    fn from(id: u16) -> Self {
        AccountID(id)
    }
}

impl From<&AccountID> for u16 {
    fn from(id: &AccountID) -> Self {
        id.0
    }
}

#[derive(PartialEq, Debug)]
pub enum State {
    Active,
    Frozen,
}

impl Default for State {
    fn default() -> Self {
        State::Active
    }
}

#[derive(Default, Debug)]
pub struct Account {
    state: State,

    available_balance: Amount,

    held_balance: Amount,
}

impl Account {
    pub fn is_frozen(&self) -> bool {
        self.state == State::Frozen
    }

    pub fn freeze(&mut self) {
        self.state = State::Frozen;
    }

    pub fn deposit(&mut self, amount: &Amount) {
        if !self.is_frozen() {
            self.available_balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: &Amount) {
        if self.is_frozen() || &self.available_balance < amount {
            return;
        }

        self.available_balance -= amount;
    }

    pub fn total_balance(&self) -> Amount {
        Amount::from(*self.available_balance + *self.held_balance)
    }

    pub fn available_balance(&self) -> &Amount {
        &self.available_balance
    }

    pub fn available_balance_mut(&mut self) -> &mut Amount {
        &mut self.available_balance
    }

    pub fn held_balance(&self) -> &Amount {
        &self.held_balance
    }

    pub fn held_balance_mut(&mut self) -> &mut Amount {
        &mut self.held_balance
    }
}
