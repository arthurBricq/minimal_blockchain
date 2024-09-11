use std::collections::HashMap;
use crate::block::Block;
use crate::simple_transaction::SimpleTransaction;

pub struct Blockchain {
    chain: Vec<Block>,
    /// A hypothesis starts from an index 'n' and contains a chain of blocks.
    /// Each hypothesis is keyed by the block hash at which divergence started.
    pending_forks: HashMap<String, Vec<Block>>,
}

impl Blockchain {
    /// Creates a new blockchain, containing a single block, the genesis.
    pub fn new() -> Self {
        let genesis = Block::genesis();
        // This nonce was generated for a difficulty of 5 zeros
        Self {
            chain: vec![genesis],
            pending_forks: HashMap::new()
        }
    }
    
    pub fn add_block_unsafe(&mut self, block: Block) {
        self.chain.push(block);
    }

    /// Returns true if the main chain was updated.
    pub fn add_block_safe(&mut self, block: Block) -> bool {
        // The previous hash is the key that indicates where this block is linked.
        let previous_hash = block.previous_hash().unwrap();
        // Try to add this block to the main chain
        if previous_hash == self.chain.last().unwrap().hash() {
            self.chain.push(block);
            true
        } else {
            for (_, chain) in &mut self.pending_forks {
                // Try to place this block on the given chain
                if previous_hash == chain.last().unwrap().hash() {
                    chain.push(block);
                    return false;
                }
            }

            // If we arrived here, it means that not a single hypothesis could accept the new block
            // So we create a new one.
            self.pending_forks.insert(previous_hash, vec![block]);
            return false;
        }
    }

    /// We check all the hypothesis over our main chain.
    /// If one of them is longer, then we switch to this one.
    pub fn resolve_pending_forks(&mut self) {
        let len = self.chain.len() - 1; 
         
        // Find the longest chain among the forks
        let best_fork = self.pending_forks
            .iter()
            .max_by_key(|(_, chain)|
                chain.last().map(|block| block.index_in_chain()).unwrap_or(0)
            );
        
        
        if best_fork.is_some() {
            // Remove the best fork from the pending ones.
            // This allows us to get ownership.
            let start = best_fork.unwrap().0.clone();
            let new_chain = self.pending_forks.remove(&start).unwrap();
            let new_len = new_chain.last().map(|b| b.index_in_chain()).unwrap_or(0) as usize;
            
            // Check if the longest chain is longer than our main branch
            if new_len > len {
                // Find the common root
                if let Some(root) = self.chain.iter().position(|b| b.hash() == start) {
                    // Remove everything after the root
                    self.chain.truncate(root + 1);

                    // Add the entire new chain
                    for block in new_chain {
                        self.chain.push(block)
                    }
                }
            }
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
        log::info!("Chain size          : {}", self.len());
        self.chain.iter().for_each(|b| log::info!("     ^ {:} --> {:?}", b.hash(), b.transactions()))
    }

}


#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    use crate::simple_transaction::SimpleTransaction;

    #[test]
    fn test_blockchain_divergence_when_divergent_chain_is_longer_at_resolution() {
        let mut chain = Blockchain::new();
        
        // Create a first block and add it to the chain
        let b1 = chain.get_candidate_block(SimpleTransaction::new());
        chain.add_block_safe(b1);
        
        // Create two blocks on top of B1
        let b2 = chain.get_candidate_block(SimpleTransaction::from_str("left"));
        let b3 = chain.get_candidate_block(SimpleTransaction::from_str("right"));
        
        // Add one of them first
        chain.add_block_safe(b2);
        
        // You can't add the next one
        assert_eq!(false, chain.add_block_safe(b3.clone()));
        
        // But b3 should be stored in a pending fork.
        assert_eq!(1, chain.pending_forks.len());
        
        // Create a new block on top of b3
        let b4 = Block::new_after_block(SimpleTransaction::from_str("I was easy to mine..."), &b3);
        
        // This one too should not be merged.
        assert_eq!(false, chain.add_block_safe(b4.clone()));
        
        // There should still be only a single pending fork
        assert_eq!(1, chain.pending_forks.len());
        
        // If we ask for a resolution now, the main chain must be swap and must now have one more block
        assert_eq!(3, chain.len());
        chain.resolve_pending_forks();
        assert_eq!(4, chain.len());
    }
}