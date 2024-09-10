use crate::block::Block;
use crate::client::Client;
use crate::transaction::Transaction;

mod block;
mod mining;
mod client;
mod pool;
mod server;
mod transaction;

fn main() {
    const DIFFICULTY: usize = 3;

    // Create three clients
    let mut client1 = Client::new(100);
    let mut client2 = Client::new(100);
    println!("Clients are created");

    let tx1 = client1.emit_transaction(client2.public_key(), 10).unwrap();
    let tx2 = client2.emit_transaction(client1.public_key(), 10).unwrap();

    let mut genesis = Block::new(vec![tx1]);
    let hash= mining::mine(&mut genesis, DIFFICULTY);
    println!("result =  {hash}");
    println!("block  =  {genesis:?}");

    // block is mined now...
    let mut next = Block::new(vec![tx2]);
    next.set_previous_hash(hash);
    let hash = mining::mine(&mut next, DIFFICULTY);
    println!("result =  {hash}");
    println!("block  =  {next:?}");

    /*
     */

    println!("Hello, world!");
}
