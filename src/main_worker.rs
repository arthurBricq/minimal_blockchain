use std::error::Error;
use crate::worker::Worker;

mod worker;

fn main() -> Result<(), Box<dyn Error>> {
    let worker = Worker::new();
    
    // The worker has to have several threads...
    // - One thread to be mining.
    // - One thread ready to interrupt the mining if another miner found a solution first.
    

    // First, register into the server
    let resp = reqwest::blocking::get("http://localhost:8000/register_worker/11")?.text()?;
    println!("Received: {:#?}", resp);

    loop {

    }

}