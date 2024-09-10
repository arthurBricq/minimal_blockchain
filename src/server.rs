use crate::simple_transaction::SimpleTransaction;
use std::collections::VecDeque;
use rand::Rng;
use crate::block::Block;
use crate::blockchain::Blockchain;

pub struct Server {
    /// Pool of pending transactions
    mempool: VecDeque<SimpleTransaction>,
    /// The current best blockchain, containing all the data
    blockchain: Blockchain
}

impl Server {
    pub fn new() -> Self {
        Self {
            mempool: VecDeque::new(),
            blockchain: Blockchain::new()
        }
    }

    pub fn submit_transaction(&mut self, tx: SimpleTransaction) {
        self.mempool.push_back(tx)
    }

    pub fn get_pending_transaction(&self) -> Option<SimpleTransaction> {
        if self.mempool.is_empty() {
            None
        } else {
            let num = rand::thread_rng().gen_range(0..self.mempool.len());
            Some(self.mempool[num].clone())
        }
    }
    
    /// Adds a block in the internal chain and remove the transaction from the poo
    pub fn add_block_safe(&mut self, block: Block) -> bool {
        if self.blockchain.add_block_safe(block) {
            let transaction_to_remove = self.blockchain.last_transaction();
            if let Some(index_to_remove) = self.mempool.iter().position(|tx| tx == transaction_to_remove) {
                self.mempool.remove(index_to_remove);
            }
            true
        } else {
            false
        }
    }
}