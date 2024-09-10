use crate::client::Client;
use crate::transaction::Transaction;
use std::collections::VecDeque;

pub struct Server {
    /// All the registered clients in the network
    // clients: Vec<Client>,
    /// addresses of the workers
    workers: Vec<String>,
    /// Pool of pending transactions
    pool: VecDeque<Transaction>
}

impl Server {
    pub fn new() -> Self {
        Self {
            // clients: vec![],
            workers: vec![],
            pool: VecDeque::new(),
        }
    }

    pub fn submit_transaction(&mut self, tx: Transaction) {
        self.pool.push_back(tx)
    }

    pub fn workers(&self) -> &Vec<String> {
        &self.workers
    }

    pub fn register_worker(&mut self, worker: String) {
        self.workers.push(worker);
    }
}