use repyh::block::Block;
use repyh::mining;
use repyh::simple_transaction::SimpleTransaction;

fn main() {
    const DIFFICULTY: usize = 5;
 
    let tx1 = SimpleTransaction::from_str("");
    let tx2 = SimpleTransaction::from_str("Arthur bought a book from Arnaud");

    // Mine the first block
    let mut genesis = Block::genesis();
    let hash = mining::mine(&mut genesis, DIFFICULTY);
    println!("result =  {hash}");
    println!("block  =  {genesis:?}");

    // Mine the second block
    let mut next = Block::new_after_block(tx2, &genesis);
    let hash = mining::mine(&mut next, DIFFICULTY);
    println!("\nresult =  {hash}");
    println!("block  =  {next:?}");
}
