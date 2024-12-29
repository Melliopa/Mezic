// src/ledger/state.rs

use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub accounts: HashMap<String, u64>, // Account balances
}

impl State {
    pub fn new() -> Self {
        State {
            accounts: HashMap::new(),
        }
    }

    pub fn update_balance(&mut self, account: String, amount: u64) {
        *self.accounts.entry(account).or_insert(0) += amount;
    }

    pub fn get_balance(&self, account: &str) -> u64 {
        *self.accounts.get(account).unwrap_or(&0)
    }
}
