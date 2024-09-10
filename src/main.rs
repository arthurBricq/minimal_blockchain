use crate::blockchain::{Block, BlockChain};

mod blockchain;
mod mining;

fn main() {
    println!("Hello, world!");

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


}
