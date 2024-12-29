// src/shards/cross_shard.rs

use crate::shards::shard_manager::Shard;

pub fn handle_cross_shard_transaction(
    sender_shard: &mut Shard,
    receiver_shard: &mut Shard,
    transaction: String,
    amount: u64,
) {
    println!("Handling cross-shard transaction: {}", transaction);

    // Update sender shard state
    if let Some(balance) = sender_shard.state.accounts.get_mut("Sender") {
        if *balance >= amount {
            *balance -= amount;
        } else {
            println!("Insufficient balance in sender shard!");
            return;
        }
    }

    // Update receiver shard state
    *receiver_shard.state.accounts.entry("Receiver".to_string()).or_insert(0) += amount;

    println!(
        "Cross-shard transaction processed between shards {} and {}",
        sender_shard.shard_id, receiver_shard.shard_id
    );
}
