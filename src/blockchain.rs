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
    
    pub fn add_block_unsafe(&mut self, block: Block) {
        self.chain.push(block);
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

    /// Returns a block at the last stage of the chain ready to be mined
    pub fn get_candidate_block(&self, transaction: SimpleTransaction) -> Block {
        Block::new_after_block(transaction, self.chain.last().unwrap())
    }
    
    pub fn len(&self) -> usize {
        self.chain.len()
    }
    
}
