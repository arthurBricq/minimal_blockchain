use std::error::Error;

/// A client to submit a transaction to the server
fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("http://localhost:8000/register_worker/11")?.text()?;
    println!("Received: {:#?}", resp);
    Ok(())
}