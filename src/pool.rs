use std::collections::VecDeque;
use crate::transaction::Transaction;

/// A pool of pending transactions
pub struct MemPool {
    buffer: VecDeque<Transaction>
}

impl MemPool {
    pub fn new() -> Self {
        Self { buffer: VecDeque::new() }
    }

    pub fn submit_transaction(&mut self, transaction: Transaction) {
        self.buffer.push_back(transaction);
    }
}