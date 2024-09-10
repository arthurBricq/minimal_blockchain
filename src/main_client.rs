use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let resp = reqwest::blocking::get("http://localhost:8000")?.text()?;
    println!("Received: {:#?}", resp);
    Ok(())
}