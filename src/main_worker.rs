use std::error::Error;
use repyh::simple_transaction::SimpleTransaction;

mod worker;
mod p2p_network;

async fn async_req(url: &str, client: reqwest::Client) -> Result<reqwest::Response, Box<dyn Error>> {
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(180))
        .send()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a thread that listens to the P2P network
    // This allows us to know if another node found a node, and if so, to check it...
    let future = p2p_network::join_p2p_network().expect("TODO: panic message");

    // Ask the server for pending transaction
    let client = reqwest::Client::new();

    loop {
        // Make a request to the server
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
                let as_text = res.text().await?;
                let parsed: SimpleTransaction = serde_json::from_str(&as_text).unwrap();
                println!("received: {:?}", parsed);
            }
            _ => {}
        }


        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}