use std::error::Error;

/// A client to submit a transaction to the server
fn main() -> Result<(), Box<dyn Error>> {
    
    if let Some(text) = std::env::args().nth(1) {
        println!("Sending to server: {text}");
        let resp = reqwest::blocking::get(format!("http://localhost:8000/submit_transaction/{}", text))?.text()?;
        println!("Received: {:#?}", resp);
    }
    
    Ok(())
}