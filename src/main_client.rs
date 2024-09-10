use std::error::Error;
use repyh::client::Client;

/// A client has a wallet and asks to do transmissions to the server
fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("http://localhost:8000/register_worker/11")?.text()?;
    println!("Received: {:#?}", resp);
    Ok(())
}