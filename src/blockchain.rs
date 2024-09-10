use crate::block::Block;
use crate::simple_transaction::SimpleTransaction;

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    /// Creates a new blockchain, containing a single block, the genesis.
    pub fn new() -> Self {
        let mut genesis = Block::genesis(SimpleTransaction::from_str(""));
        // This nonce was generated for a difficulty of 5 zeros
        genesis.set_nonce(1293653);
        Self {
            chain: vec![genesis]
        }
    }

    pub fn add_block_safe(&mut self, block: Block) -> bool {
        if block.previous_hash().unwrap() == self.chain.last().unwrap().hash() {
            self.chain.push(block);
            true
        } else {
            false
        }
    }
    
    pub fn last_transaction(&self) -> &SimpleTransaction {
        self.chain.last().unwrap().transactions()
    }
    
}
