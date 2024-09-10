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
    
    pub fn get_last_transaction(&self, n: u32) -> Vec<SimpleTransaction> {
        let mut result = Vec::new();
        let size = self.pool.len();
        for i in 0..n {
            if i >= 0 {
                result.push(self.pool[size - 1 - i].clone());
            }
        }
        result
    }

}