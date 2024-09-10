use crate::block::Block;

mod block;
mod mining;
mod client;
mod pool;
mod server;
mod transaction;

fn main() {

    /*
    let mut gen = Block::new(145);
    let hash= mining::mine(&mut gen);
    println!("result =  {hash}");
    println!("block  =  {gen:?}");

    // block is mined now...
    let mut next = Block::new(123);
    next.set_previous_hash(hash);
    let hash = mining::mine(&mut next);
    println!("result =  {hash}");
    println!("block  =  {next:?}");
     */

    println!("Hello, world!");



}
