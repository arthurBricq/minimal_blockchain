use crate::client::Client;
use crate::pool::MemPool;

pub struct Server {
    /// All the registered clients in the network
    clients: Vec<Client>,
    /// Pool of pending transactions
    pool: MemPool
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: vec![],
            pool: MemPool::new(),
        }
    }
}