extern crate log;

use repyh::block::Block;
use repyh::simple_transaction::SimpleTransaction;
use reqwest::Client;
use std::error::Error;
use std::sync::{Arc, Mutex};
use env_logger::Env;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use repyh::blockchain::Blockchain;
use repyh::mining::mine;

mod p2p_network;

const DIFFICULTY: usize = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let (tx_local_block, rx_local_block) = mpsc::unbounded_channel();
    let (tx_network_blocks, mut rx_network_blocks) = mpsc::unbounded_channel::<String>();

    // Create a thread that listens to the P2P network
    // This allows us to know if another node found a node, and if so, to check it...
    p2p_network::join_p2p_network(rx_local_block, tx_network_blocks).expect("TODO: panic message");
    
    // Leave some initial time so that the P2P network setup correctly
    log::info!("P2P initialized. Waiting for a small delay for initialization to finish...");
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    
    let chain = Arc::new(Mutex::new(Blockchain::new()));
    let client = reqwest::Client::new();

    loop {
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        let cloned_client = client.clone();
        let cloned_tx = tx_local_block.clone();
        let cloned_chain = chain.clone();

        let (tx1, rx1) = oneshot::channel();

        // Create a new task, but don't await on the task
        tokio::spawn(async move {
            request_transaction_and_mine(cloned_tx, cloned_client, cloned_token, cloned_chain, tx1).await;
        });

        tokio::select! {
            Some(msg) = rx_network_blocks.recv() => {
                // New block received from the network
                // Extract the new block
                let block: Block = serde_json::from_str(&msg).unwrap();
                if block.is_hash_valid(DIFFICULTY) {
                    log::info!("Block from network arrived.");
                    block.print_block();
                    if chain.lock().unwrap().add_block_safe(block) {
                        // This means we accept the block from another worker.
                        log::info!("cancellation accepted.");
                        token.cancel();
                    } else {
                        log::error!("cancellation rejected.");
                    }
                    chain.lock().unwrap().print_chain();
                }
                
            }
            // This branch is necessary to 'listen' for mining finished
            val = rx1 => {}
        }
    }


}

/// * Ask the transaction server for a new transaction to mine
/// * Start to mine while listening for cancellation
/// * If mining finished, forward your block to the network
async fn request_transaction_and_mine(
    tx_local_block: UnboundedSender<String>,
    client: Client,
    cancellation_token: CancellationToken,
    chain: Arc<Mutex<Blockchain>>,
    tx1: oneshot::Sender<()>,
) -> Result<(), Box<dyn Error>> 
{

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
            log::info!("Received new transaction: {parsed:?}");

            // Start to mine the block
            // We use a cancellation token to abort the task
            let mut new_block = chain.lock().unwrap().get_candidate_block(parsed);
            if let Some(_) = mine(&mut new_block, DIFFICULTY, cancellation_token.clone()).await {
                log::info!("  Finished to mine !");
                new_block.print_block();

                // Broadcast the mined bitcoin to the swarm.
                tx_local_block
                    .send(serde_json::to_string(&new_block).unwrap())
                    .expect("Broadcasting mined block did not work.");

                // Communicate to the server that this block was mined.
                tx1.send(()).unwrap();

                // Set it in the chain.
                chain.lock().unwrap().add_block_unsafe(new_block);

                chain.lock().unwrap().print_chain();

                // Swap the token...
                cancellation_token.cancel();
            }
        }
        _ => panic!("wtf")
    }
    
    Ok(())
}

/// Sends http request in async rust
async fn async_req(url: &str, client: reqwest::Client) -> Result<reqwest::Response, Box<dyn Error>> {
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(180))
        .send()
        .await?;
    Ok(response)
}
