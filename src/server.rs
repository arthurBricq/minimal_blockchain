use crate::simple_transaction::SimpleTransaction;
use std::collections::VecDeque;

pub struct Server {
    /// Pool of pending transactions
    pool: VecDeque<SimpleTransaction>
}

impl Server {
    pub fn new() -> Self {
        Self {
            pool: VecDeque::new(),
        }
    }

    pub fn submit_transaction(&mut self, tx: SimpleTransaction) {
        self.pool.push_back(tx)
    }
    
    pub fn get_last_transaction(&self) -> Option<SimpleTransaction> {
        if self.pool.is_empty() {
            None
        } else {
            Some(self.pool[self.pool.len() - 1].clone())
        }
    }

}