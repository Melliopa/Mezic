// src/shards/shard_manager.rs

use std::collections::HashMap;
use super::cross_shard::handle_cross_shard_transaction;
use crate::ledger::transactions::Transaction;
use crate::ledger::state::State;

pub struct Shard {
    pub shard_id: u64,
    pub transactions: Vec<Transaction>,
    pub state: State,
}

impl Shard {
    pub fn new(shard_id: u64) -> Self {
        Shard {
            shard_id,
            transactions: Vec::new(),
            state: State::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        println!("Adding transaction to shard {}: {:?}", self.shard_id, transaction);
        self.transactions.push(transaction);
    }

    pub fn process_transactions(&mut self) {
        println!("Processing transactions in shard {}", self.shard_id);
        for tx in &self.transactions {
            self.state.update_balance(tx.receiver.clone(), tx.amount);
            // Subtract from sender
            let sender_balance = self.state.get_balance(&tx.sender);
            if sender_balance >= tx.amount {
                self.state.accounts.insert(tx.sender.clone(), sender_balance - tx.amount);
            } else {
                println!("Insufficient balance for transaction: {:?}", tx);
            }
        }
        self.transactions.clear();
    }
}

pub struct ShardManager {
    pub shards: HashMap<u64, Shard>,
}

impl ShardManager {
    pub fn new(num_shards: u64) -> Self {
        let mut shards = HashMap::new();
        for shard_id in 0..num_shards {
            shards.insert(shard_id, Shard::new(shard_id));
        }
        ShardManager { shards }
    }

    pub fn route_transaction(&mut self, transaction_str: String, shard_id: u64) {
        // Deserialize transaction
        let transaction: Transaction = serde_json::from_str(&transaction_str)
            .expect("Failed to deserialize transaction");
        if let Some(shard) = self.shards.get_mut(&shard_id) {
            shard.add_transaction(transaction);
        } else {
            println!("Shard {} not found!", shard_id);
        }
    }

    pub fn process_all_shards(&mut self) {
        for shard in self.shards.values_mut() {
            shard.process_transactions();
        }
    }

    pub fn create_block(&self, validator: String) -> crate::ledger::transactions::Block {
        // Collect transactions from all shards
        let mut all_transactions = Vec::new();
        for shard in self.shards.values() {
            all_transactions.extend(shard.transactions.clone());
        }

        // Create a new block (for simplicity, using placeholder values)
        let new_block = crate::ledger::transactions::Block::new(
            1, // Placeholder index
            "prev_hash_placeholder".to_string(),
            all_transactions,
        );

        new_block
    }
}
