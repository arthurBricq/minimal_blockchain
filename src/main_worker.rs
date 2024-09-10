use repyh::block::Block;
use repyh::simple_transaction::SimpleTransaction;
use reqwest::Client;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use repyh::blockchain::Blockchain;

mod worker;
mod p2p_network;

/// Sends http request in async rust
async fn async_req(url: &str, client: reqwest::Client) -> Result<reqwest::Response, Box<dyn Error>> {
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(180))
        .send()
        .await?;
    Ok(response)
}

/// Find the nonce for which the bytes of the given block match a pattern
/// starting with N zeros, where `N` is the `difficulty` argument.
pub async fn mine(
    block: &mut Block,
    difficulty: usize,
    cancellation_token: CancellationToken
) -> Option<String> {
    let start_pattern = String::from_utf8(vec![b'0'; difficulty]).unwrap();

    // We are looking for an output that starts with a certain number of zeros
    for nonce in 0..u64::MAX {
        // look for a start with N zeros
        block.set_nonce(nonce);
        let hash = block.hash();
        if hash.starts_with(&start_pattern) {
            return Some(hash)
        }
        // Always check if this thread was asked to be cancelled
        if cancellation_token.is_cancelled() {
            return None
        }
    }

    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx_local_block, rx_local_block) = mpsc::unbounded_channel();
    let (tx_network_blocks, mut rx_network_blocks) = mpsc::unbounded_channel();

    // Create a thread that listens to the P2P network
    // This allows us to know if another node found a node, and if so, to check it...
    p2p_network::join_p2p_network(rx_local_block, tx_network_blocks).expect("TODO: panic message");
    
    // Leave some initial time so that the P2P network setup correctly
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    
    let chain = Arc::new(Mutex::new(Blockchain::new()));
    let client = reqwest::Client::new();
    let token = CancellationToken::new();

    loop {
        let cloned_token = token.clone();
        let cloned_client = client.clone();
        let cloned_tx = tx_local_block.clone();
        let cloned_chain = chain.clone();
        
        tokio::select! {
            Some(msg) = rx_network_blocks.recv() => {
                // Extract the new block
                let block: Block = serde_json::from_str(&msg).unwrap();
                println!("RECEIVED A BLOCK FOR CANCELLATION");
                println!("{block:?}");
                token.cancel();
            }
            _ = request_new_transaction_and_work(cloned_tx, cloned_client, cloned_token, cloned_chain) => {
                println!("Mining went well :)")
            }
        }
        
    }
}

async fn request_new_transaction_and_work(
    tx_local_block: UnboundedSender<String>,
    client: Client,
    cancellation_token: CancellationToken,
    chain: Arc<Mutex<Blockchain>>
) -> Result<(), Box<dyn Error>> {
    // Ask the server for pending transaction
    let response = if let Ok(response) = async_req("http://localhost:8000/get_transaction", client).await {
        Some(response)
    } else {
        None
    };

    match response {
        Some(res) => {
            // decrypt the transaction
            let as_text = res.text().await?;
            let parsed: SimpleTransaction = serde_json::from_str(&as_text).unwrap();
            println!("Received new transaction: {parsed:?}");

            // Start to mine the block
            // We use a cancellation token to abort the task
            let mut new_block = chain.lock().unwrap().get_candidate_block(parsed);
            if let Some(hash) = mine(&mut new_block, 5, cancellation_token).await {
                println!("Finished to mine   : {hash}");
                println!("Finished to block ! : {new_block:?}");
                
                // Broadcast the mined bitcoin to the swarm.
                tx_local_block
                    .send(serde_json::to_string(&new_block).unwrap())
                    .expect("Broadcasting mined block did not work.");
                
                // Communicate to the server that this block was mined.
                
                // Set it in the chain.
                chain.lock().unwrap().add_block_unsafe(new_block)
            }
        }
        _ => panic!("wtf")
    }
    
    Ok(())
}