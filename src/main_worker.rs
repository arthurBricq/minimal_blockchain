use std::error::Error;
use sha256::digest;
use tokio::select;
use tokio::sync::mpsc;
use repyh::block::Block;
use repyh::simple_transaction::SimpleTransaction;
use tokio_util::sync::CancellationToken;

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
pub async fn mine(block: &mut Block, difficulty: usize, cancellation_token: CancellationToken) -> String {
    let start_pattern = String::from_utf8(vec![b'0'; difficulty]).unwrap();

    // We are looking for an output that starts with a certain number of zeros
    for nonce in 0..u64::MAX {
        block.set_nonce(nonce);
        let data = block.bytes();
        let hash = digest(data);

        // look for a start with N zeros
        if hash.starts_with(&start_pattern) {
            return hash
        }

        if cancellation_token.is_cancelled() {
            // Abort
            panic!("to abort...")
        }
    }

    panic!("")
}

async fn mine_something(cancellation_token: CancellationToken) -> String {
    // Mining a block here.
    let tx1 = SimpleTransaction::from_str("Victor bought a car from Arthur");
    let mut genesis = Block::new(tx1);
    println!("Starting to mine");
    mine(&mut genesis, 6, cancellation_token).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::unbounded_channel();

    // Start mining on another task

    // Create a thread that listens to the P2P network
    // This allows us to know if another node found a node, and if so, to check it...
    p2p_network::join_p2p_network().expect("TODO: panic message");

    let client = reqwest::Client::new();

    // Ask the server for pending transaction
    let new_client = client.clone();
    let response = tokio::spawn(async move {
        if let Ok(response) = async_req("http://localhost:8000/get_transaction", new_client).await {
            Some(response)
        } else {
            None
        }
    });

    match response.await {
        Ok(Some(res)) => {
            // decrypt the transaction
            let as_text = res.text().await?;
            let parsed: SimpleTransaction = serde_json::from_str(&as_text).unwrap();
            println!("Received new transaction: {parsed:?}");

            // Start to mine the block
            // We use a cancellation token to abort the task
            let token = CancellationToken::new();
            let cloned_token = token.clone();
            tokio::task::spawn(async move {
                let mut new_block = Block::new(parsed);
                let hash = mine(&mut new_block, 5, cloned_token).await;
                println!("Finished to mine ! : {hash}");
                
                // Broadcast the mined bitcoin to the swarm
                tx.send(serde_json::to_string(&new_block).unwrap()).expect("Broadcasting mined block did not work.");
            });
            

        }
        _ => {}
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("Hello...")
    }
    Ok(())
}