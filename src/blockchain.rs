use std::cmp::max;
use std::collections::{HashMap, VecDeque};
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
    /// A pool of blocks that worker received but that can't be attached to no other. 
    orphan: VecDeque<Block>
}

impl Blockchain {
    /// Creates a new blockchain, containing a single block (the genesis)
    pub fn new() -> Self {
        let genesis = Block::genesis();
        Self {
            chain: vec![genesis],
            pending_forks: HashMap::new(),
            orphan: VecDeque::new()
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
        let respoonse = if previous_hash == self.chain.last().unwrap().hash() {
            self.chain.push(block);
            true
        } else {
            // Try to place this block at the head of one of the forked chain
            for (_, chain) in &mut self.pending_forks {
                // Try to place this block on the given chain
                if previous_hash == chain.last().unwrap().hash() {
                    chain.push(block);
                    return false;
                }
            }

            // If we arrived here, it means that
            // - not a single hypothesis could accept the new block at his head)
            // - the main chain could not
            // There are two possibilities now
            // 1. This block is the start of a new fork
            //    In this case, we can check whether the previous hash is in the main chain
            // 2. This block was received 'too' early and is not attached to any of the previous
            //    block. This happens when the communication fails. In this case, we store it
            //    and will try later on to fit it somewhere

            let is_new_fork = self.chain.iter().any(|block| block.hash() == previous_hash);
            if is_new_fork {
                self.pending_forks.insert(previous_hash, vec![block]);
            } else {
                self.orphan.push_back(block);
            }

            false
        };
        
        
        
        
        respoonse
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
        // TODO uncomment these lines, for now I am trying to debug & this line could be the reason
        //      of some problems.G

        // self.pending_forks.retain(|_, chain| {
        //     let chain_len = chain.last().map(|block| block.index_in_chain()).unwrap_or(0);
        //     chain_len as i64 > len as i64 - SAFE_HORIZON
        // });

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
        fn print_single_chain(start: usize, blocks: &[Block]) {
            for (i, b) in blocks.iter().enumerate() {
                log::info!("       ({})    {:?}", i + start, b.transactions())
            }
        }

        log::info!("Main chain");
        print_single_chain(0, &self.chain);
        self.pending_forks.iter().for_each(|(start_hash, blocks)| {
            if let Some(root) = self.chain.iter().position(|b| &b.hash() == start_hash) {
                log::info!(" ~ new fork");
                print_single_chain(root, blocks);
            }
        });
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

    #[test]
    fn test_divergence_with_unordered_buffer() {

        // Create a chain
        let mut chain = Blockchain::new();
        let b1 = chain.get_candidate_block(SimpleTransaction::new());
        chain.add_block_safe(b1.clone());

        // Create three block on top of each others
        let b2 = Block::new_after_block(SimpleTransaction::from_str("1"), &b1);
        let b3 = Block::new_after_block(SimpleTransaction::from_str("1"), &b2);

        // If you send `b3` before `b2`, the main chain must not be updated of course
        assert_eq!(2, chain.len());
        chain.add_block_safe(b3);
        assert_eq!(2, chain.len());
        
        // We can now check that we have 1 orphan block
        assert_eq!(1, chain.orphan.len());

        // But after you send `b2`, the chain must not be of size '3' but indeed of size '4'
        // It should detect that it can create a new chain longer
        chain.add_block_safe(b2);
        assert_eq!(4, chain.len());
    }
}