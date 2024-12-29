// src/consensus/pos.rs

use std::collections::HashMap;

pub struct ProofOfStake {
    pub validators: HashMap<String, u64>, // Validator address and their stake
}

impl ProofOfStake {
    pub fn new() -> Self {
        ProofOfStake {
            validators: HashMap::new(),
        }
    }

    pub fn register_validator(&mut self, address: String, stake: u64) {
        self.validators.insert(address, stake);
        println!("Validator {} registered with stake {}", address, stake);
    }

    pub fn select_validator(&self) -> Option<String> {
        // Simple selection: choose validator with the highest stake
        self.validators
            .iter()
            .max_by_key(|entry| entry.1)
            .map(|(address, _)| address.clone())
    }

    pub fn record_block(&mut self, validator: String) {
        // Placeholder for recording block production (e.g., reward distribution)
        println!("Recording block production for validator {}", validator);
    }
}
