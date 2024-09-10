// Taken from https://github.com/libp2p/rust-libp2p/blob/master/examples/chat/src/main.rs

use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, Swarm};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tracing_subscriber::EnvFilter;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct P2PBlockSharingBehavior {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

pub fn join_p2p_network(
    rx_local_blocks: UnboundedReceiver<String>,
    tx_network_blocks: UnboundedSender<String>
) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = build_libp2p_swarm()?;

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("blockchain-net");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Spawn a new thread of this the P2P network
    let future = tokio::spawn(async move {
        handle_swarm(swarm, topic, rx_local_blocks, tx_network_blocks).await;
    });

    Ok((future))
}

async fn handle_swarm(
    mut swarm: Swarm<P2PBlockSharingBehavior>,
    topic: gossipsub::IdentTopic,
    mut rx_local_blocks: UnboundedReceiver<String>,
    mut tx_network_blocks: UnboundedSender<String>
) {
    println!("Joining swarm ...");

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Kick it off
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(topic.clone(), line.as_bytes()) {
                    println!("Publish error: {e:?}");
                }
            }
            Some(msg) = rx_local_blocks.recv() => {
                if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(topic.clone(), msg.as_bytes()) {
                    println!("Publish error: {e:?}");
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(P2PBlockSharingBehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(P2PBlockSharingBehaviorEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(P2PBlockSharingBehaviorEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message})) => {
                    let msg = String::from_utf8_lossy(&message.data).to_string();
                    println!("Got message: '{msg}' with id: {id} from peer: {peer_id}");
                    // Communicate to the working thread that another node is proposing a node...
                    tx_network_blocks.send(msg).unwrap();
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }

}

fn build_libp2p_swarm() -> Result<Swarm<P2PBlockSharingBehavior>, Box<dyn Error>> {
    let swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(P2PBlockSharingBehavior { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}