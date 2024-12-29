// src/networking/libp2p.rs

use libp2p::{
    identity, mdns, swarm::SwarmBuilder, PeerId, Swarm, Multiaddr, NetworkBehaviour,
    gossipsub::{self, Gossipsub, GossipsubConfig, GossipsubMessage, MessageAuthenticity, Topic},
};
use tokio::sync::mpsc;
use futures::StreamExt;

#[derive(NetworkBehaviour)]
pub struct BlockchainBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: mdns::Mdns,
}

pub struct BlockchainNetwork {
    pub swarm: Swarm<BlockchainBehaviour>,
    pub topic: Topic,
}

impl BlockchainNetwork {
    pub fn new() -> (Self, mpsc::Receiver<GossipsubMessage>) {
        // Generate a key pair for this node
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        println!("Local Peer ID: {}", local_peer_id);

        // Configure Gossipsub
        let gossipsub_config = GossipsubConfig::default();
        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .expect("Failed to create Gossipsub");

        // Create a topic for blockchain messages
        let topic = Topic::new("blockchain");

        // Subscribe to the topic
        gossipsub.subscribe(&topic).unwrap();

        // Configure mDNS for local peer discovery
        let mdns = mdns::Mdns::new(mdns::MdnsConfig::default()).expect("Failed to create mDNS");

        // Combine behaviours
        let behaviour = BlockchainBehaviour { gossipsub, mdns };

        // Build the swarm
        let mut swarm = SwarmBuilder::new(behaviour, local_key, local_peer_id.clone())
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        // Create a channel for incoming messages
        let (tx, rx) = mpsc::channel(32);
        let mut swarm_clone = swarm.clone();

        // Listen for incoming messages
        tokio::spawn(async move {
            while let Some(event) = swarm_clone.next().await {
                match event {
                    libp2p::swarm::SwarmEvent::Behaviour(BlockchainBehaviourEvent::Gossipsub(
                        gossipsub_event,
                    )) => match gossipsub_event {
                        gossipsub::GossipsubEvent::Message { message, .. } => {
                            tx.send(message).await.expect("Failed to send message");
                        }
                        _ => {}
                    },
                    libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {:?}", address);
                    }
                    _ => {}
                }
            }
        });

        (
            BlockchainNetwork {
                swarm,
                topic,
            },
            rx,
        )
    }

    pub async fn send_message(&mut self, message: String) {
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.topic.clone(), message.as_bytes())
            .expect("Failed to publish message");
    }

    pub fn listen_on(&mut self, addr: Multiaddr) {
        Swarm::listen_on(&mut self.swarm, addr).expect("Failed to start listening");
    }

    pub async fn handle_message(&self, message: GossipsubMessage) {
        let payload = String::from_utf8_lossy(&message.data);
        if payload.starts_with("TX:") {
            println!("Received Transaction: {}", &payload[3..]);
            // TODO: Add transaction to the pending pool
        } else if payload.starts_with("BLOCK:") {
            println!("Received Block: {}", &payload[6..]);
            // TODO: Validate and add the block to the blockchain
        } else {
            println!("Unknown Message: {}", payload);
        }
    }
}

// Helper enum to match behaviour events
#[derive(Debug)]
pub enum BlockchainBehaviourEvent {
    Gossipsub(gossipsub::GossipsubEvent),
    Mdns(mdns::MdnsEvent),
}

impl From<gossipsub::GossipsubEvent> for BlockchainBehaviourEvent {
    fn from(event: gossipsub::GossipsubEvent) -> Self {
        BlockchainBehaviourEvent::Gossipsub(event)
    }
}

impl From<mdns::MdnsEvent> for BlockchainBehaviourEvent {
    fn from(event: mdns::MdnsEvent) -> Self {
        BlockchainBehaviourEvent::Mdns(event)
    }
}
