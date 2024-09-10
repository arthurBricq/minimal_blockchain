use std::error::Error;
use repyh::client::Client;

fn main() -> Result<(), Box<dyn Error>> {

    // Create a new client
    let mut client1 = Client::new(100);

    println!("Hello, world!");
    let resp = reqwest::blocking::get("http://localhost:8000")?.text()?;
    println!("Received: {:#?}", resp);
    Ok(())
}