use crate::block::Block;
use crate::simple_transaction::SimpleTransaction;

pub struct Blockchain {
    chain: Vec<Block>,
    /// A hypothesis starts from an index 'n' and contains a chain of blocks.
    hypothesis: Vec<(usize, Vec<Block>)>
}

impl Blockchain {
    /// Creates a new blockchain, containing a single block, the genesis.
    pub fn new() -> Self {
        let mut genesis = Block::genesis();
        // This nonce was generated for a difficulty of 5 zeros
        Self {
            chain: vec![genesis],
            hypothesis: vec![]
        }
    }
    
    pub fn add_block_unsafe(&mut self, block: Block) {
        self.chain.push(block);
    }

    /// Returns true if the main chain was updated.
    pub fn add_block_safe(&mut self, block: Block) -> bool {
        if block.previous_hash().unwrap() == self.chain.last().unwrap().hash() {
            self.chain.push(block);
            true
        } else {
            // Try to place the cube in a new hypothesis
            
            false
        }
    }
    
    pub fn last_transaction(&self) -> &SimpleTransaction {
        self.chain.last().unwrap().transactions()
    }

    /// Returns a block at the last stage of the chain ready to be mined
    pub fn get_candidate_block(&self, transaction: SimpleTransaction) -> Block {
        Block::new_after_block(transaction, self.chain.last().unwrap())
    }
    
    pub fn len(&self) -> usize {
        self.chain.len()
    }
    
    pub fn print_chain(&self) {
        println!("Chain size          : {}", self.len());
        self.chain.iter().for_each(|b| println!("     ^ {:} --> {:?}", b.hash(), b.transactions()))
    }
    
    /// In the case that 
    pub fn store_hypothesis(&mut self, block: Block) {
        let mut i = 1;
        let n = self.chain.len();
        loop {
            if i > 3 || n - i - 1 < 0{
                return 
            }
            
        }
    }
    
}
