use std::cmp::max;
use std::collections::HashMap;
use crate::block::Block;
use crate::simple_transaction::SimpleTransaction;

/// Depth below the head of a chain after which we consider that all workers must have agreed on.
const SAFE_HORIZON: i64 = 4;

/// Keeps track of the main chain and of possible divergence on the last `SAFE_HORIZON` layers.
pub struct Blockchain {
    chain: Vec<Block>,
    /// Each hypothesis is keyed by the block hash at which divergence started.
    /// The forked chain is then a list of blocks starting from this hash.
    /// TODO (optimization) store the index of the root instead of storing the hash of the root
    pending_forks: HashMap<String, Vec<Block>>,
}

impl Blockchain {
    /// Creates a new blockchain, containing a single block (the genesis)
    pub fn new() -> Self {
        let genesis = Block::genesis();
        Self {
            chain: vec![genesis],
            pending_forks: HashMap::new()
        }
    }
    
    pub fn add_block_unsafe(&mut self, block: Block) {
        self.chain.push(block);
    }

    /// Returns true if the main chain was updated, false otherwise.
    /// If the block is not set inserted in the main chain, it is kept as a hypothesis.
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
        let len = (self.chain.len() - 1) as u64;

        // Find the longest chain among the forks
        let best_fork = self.pending_forks
            .iter()
            .filter(|(start, chain)|
                chain.last().map(|block| block.index_in_chain()).unwrap_or(0) > len
            ).max_by_key(|(_, chain)|
                chain.last().map(|block| block.index_in_chain()).unwrap_or(0)
            );

        // If we have found a better fork, then perform the swapping
        if best_fork.is_some() {
            // Remove the best fork from the pending ones.
            // This allows us to get ownership.
            let start = best_fork.unwrap().0.clone();
            let new_chain = self.pending_forks.remove(&start).unwrap();
            log::error!("WE ARE SWAPPING THE MAIN BRANCH");

            if let Some(root) = self.chain.iter().position(|b| b.hash() == start) {
                // Remove everything after the root
                self.chain.truncate(root + 1);

                // Add the entire new chain
                for block in new_chain {
                    self.chain.push(block)
                }
            }
        }

        // Chain cleanup
        // We remove every pending fork that is more than N blocks behind the main head.
        self.pending_forks.retain(|_, chain| {
            let chain_len = chain.last().map(|block| block.index_in_chain()).unwrap_or(0);
            chain_len as i64 > len as i64 - SAFE_HORIZON
        });

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

    pub fn has_transaction(&self, tx: &SimpleTransaction) -> bool {
        self.chain.iter().any(|block| block.transactions() == tx)
    }
    
    /// Returns true if the transaction is written before the 'safe' horizon,
    /// which mean that we consider that all workers have agreed upon this position.
    pub fn is_transaction_safely_written(&self, tx: &SimpleTransaction) -> bool {
        if self.chain.len() < SAFE_HORIZON as usize {
            return false;
        }
        self.chain[..self.chain.len() - SAFE_HORIZON as usize].iter().any(|block| block.transactions() == tx)
    }

    pub fn print_chain(&self) {
        log::info!("Main chain size          : {}", self.len());
        self.chain.iter().for_each(|b| log::info!("     ^ {:?}", b.transactions()));
        log::info!("Number of pending forks  : {}", self.pending_forks.len());
        self.pending_forks.iter().for_each(|(_, blocks)| log::info!("  * size of fork = {}", blocks.len()));
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

        // And there should not be anymore pending forks
        assert_eq!(0, chain.pending_forks.len());
    }

    #[test]
    fn test_blockchain_divergence_when_main_chain_is_longer_at_resolution() {
        let mut chain = Blockchain::new();

        // Create a first block and add it to the chain
        let b1 = chain.get_candidate_block(SimpleTransaction::new());
        chain.add_block_safe(b1);

        // Create two blocks on top of B1
        let b2 = chain.get_candidate_block(SimpleTransaction::from_str("left"));
        let b3 = chain.get_candidate_block(SimpleTransaction::from_str("right"));

        // Add one of them first
        chain.add_block_safe(b2.clone());

        // You can't add the next one
        assert_eq!(false, chain.add_block_safe(b3));

        // But b3 should be stored in a pending fork.
        assert_eq!(1, chain.pending_forks.len());

        // Create a new block on top of b2
        let b4 = Block::new_after_block(SimpleTransaction::from_str("I was easy to mine..."), &b2);

        // This one can be merged
        assert_eq!(true, chain.add_block_safe(b4));

        // There should still be only a single pending fork
        assert_eq!(1, chain.pending_forks.len());

        // But the main branch should not change here !
        assert_eq!(4, chain.len());
        chain.resolve_pending_forks();
        assert_eq!(4, chain.len());

        // And there should still be the pending fork, because maybe some more nodes will come later on
        assert_eq!(1, chain.pending_forks.len());
    }
}