use repyh::block::Block;
use repyh::client::Client;
use repyh::mining;

fn main() {
    const DIFFICULTY: usize = 3;

    // Create three clients
    let mut client1 = Client::new(100);
    let mut client2 = Client::new(100);
    println!("Clients are created");

    let tx1 = client1.emit_transaction(client2.public_key(), 10).unwrap();
    let tx2 = client2.emit_transaction(client1.public_key(), 10).unwrap();

    // Mine the first block
    let mut genesis = Block::new(vec![tx1]);
    let hash = mining::mine(&mut genesis, DIFFICULTY);
    println!("result =  {hash}");
    println!("block  =  {genesis:?}");

    // Mine the second block
    let mut next = Block::new_from_hash(vec![tx2], hash);
    let hash = mining::mine(&mut next, DIFFICULTY);
    println!("\nresult =  {hash}");
    println!("block  =  {next:?}");

}
