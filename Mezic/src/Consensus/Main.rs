// src/main.rs

mod consensus;
mod networking;
mod ledger;
mod shards;
mod security;
mod smart_contracts;
mod storage;

use networking::BlockchainNetwork;
use shards::shard_manager::ShardManager;
use shards::cross_shard::handle_cross_shard_transaction;
use ledger::transactions::Transaction;
use ledger::state::State;
use consensus::pos::ProofOfStake;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Initialize the blockchain state and shard manager
    let num_shards = 4;
    let mut shard_manager = ShardManager::new(num_shards);

    // Initialize Proof of Stake consensus
    let mut pos = ProofOfStake::new();

    // Register validators
    pos.register_validator("Validator1".to_string(), 1000);
    pos.register_validator("Validator2".to_string(), 800);

    // Initialize the network
    let (mut blockchain_network, mut rx) = BlockchainNetwork::new();

    // Start listening on a multiaddr
    let addr: libp2p::Multiaddr = "/ip4/127.0.0.1/tcp/4001".parse().expect("Invalid multiaddr");
    blockchain_network.listen_on(addr);

    println!("Blockchain network is running...");

    // Example transactions
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 10);
    let tx2 = Transaction::new("Charlie".to_string(), "Dave".to_string(), 5);

    // Route transactions to specific shards based on some criteria (e.g., sender)
    shard_manager.route_transaction(tx1.to_string(), 1);
    shard_manager.route_transaction(tx2.to_string(), 2);

    // Broadcast transactions to the network
    blockchain_network
        .send_message(format!("TX:{}", tx1.to_string()))
        .await;
    blockchain_network
        .send_message(format!("TX:{}", tx2.to_string()))
        .await;

    // Simulate block mining by a validator
    if let Some(validator) = pos.select_validator() {
        shard_manager.process_all_shards();
        let new_block = shard_manager.create_block(validator.clone());
        blockchain_network
            .send_message(format!("BLOCK:{}", new_block.to_string()))
            .await;
        pos.record_block(validator.clone());
        println!("Block mined by {}!", validator);
    }

    // Listen for incoming messages
    while let Some(message) = rx.recv().await {
        blockchain_network.handle_message(message).await;
    }
}
