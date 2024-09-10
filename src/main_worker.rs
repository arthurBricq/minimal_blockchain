use std::error::Error;
use crate::worker::Worker;

mod worker;
mod p2p_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let worker = Worker::new();
    
    // Create a thread that listens to the P2P network
    p2p_network::join_p2p_network().expect("TODO: panic message");
    
    // First, register into the server
    let resp = reqwest::blocking::get("http://localhost:8000/register_worker/11")?.text()?;
    println!("Received: {:#?}", resp);

    loop {

    }

}