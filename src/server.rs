use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rouille::{router, Response};
use repyh::block::Block;
use repyh::blockchain::Blockchain;
use repyh::simple_transaction::SimpleTransaction;

/// Server in charge of keeping track of the pending transactions
pub struct Server {
    /// Pool of pending transactions
    mempool: VecDeque<SimpleTransaction>,
    /// The server holds a representation of blockchain that it keeps building using all the blocks
    /// received by workers.
    /// Keeping track of the blockchain allows the server to safely remove pending transactions, once
    /// they are past the safe depth, and to (in the future) provide to new workers with am up-to-date
    /// version of the chain.
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

    /// Returns one of the transaction in the pool
    fn get_pending_transaction(&self) -> Option<SimpleTransaction> {
        if self.mempool.is_empty() {
            None
        } else {
            let num = rand::thread_rng().gen_range(0..self.mempool.len());
            Some(self.mempool[num].clone())
        }
    }
    
    /// Checks if some of the transaction on the pool is safely written in the chain, 
    /// and if so remove it from the pool.
    fn resolve_safe_transactions(&mut self) {
        self.mempool.retain(|tx| !self.blockchain.is_transaction_safely_written(&tx))
    }
    
}

const ACCEPTED: &str = "Accepted";

const REJECTED: &str = "Rejected";

/// Launch a webserver, associated with a transaction server, that will answer to workers and external clients.
/// 
/// Supported end-points
/// * /submit_transaction/DATA : registers a new transaction in the mempool to be saved
/// 
/// * /get_transaction/        : returns a single transaction that is in the mempool.
/// 
/// * /submit_block/DATA       : registers a new block mined by one of the worker. 
///                              The server keeps track of the nodes submitted by the workers to 
///                              have its own version of the blockchain.
pub fn run_web_server(server: Arc<Mutex<Server>>) {
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/submit_transaction/{data: String}) => {
                // Worker ask for a random transaction in the list from the pending ones
                println!("A client submitted a new transaction: {data}");
                server.lock().unwrap().submit_transaction(SimpleTransaction::from_str(&data));
                Response::text("submitted")
            },

            (GET) (/get_transaction) => {
                // Worker ask for a random transaction in the list from the pending ones
                if let Some(transaction) = server.lock().unwrap().get_pending_transaction() {
                    let as_json = serde_json::to_string(&transaction).unwrap();
                    Response::text(as_json)
                } else {
                    Response::text("")
                }
            },

            (GET) (/submit_block/{data: String}) => {
                // Parse the block sent by the client
                let received: Block = serde_json::from_str(&data).unwrap();
                log::info!("Server received block.");
                server.lock().unwrap().blockchain.add_block_safe(received);
                server.lock().unwrap().blockchain.resolve_pending_forks();
                server.lock().unwrap().blockchain.print_chain();
                server.lock().unwrap().resolve_safe_transactions();
                log::info!("Remaining transaction in the pool: {:?}", server.lock().unwrap().mempool.len());
                Response::text("")
            },

            _ => Response::empty_404()
        )
    });

}
