use crate::simple_transaction::SimpleTransaction;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rouille::{router, Response};
use crate::block::Block;
use crate::blockchain::Blockchain;

/// Server in charge of keeping track of the pending transactions
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

const ACCEPTED: &str = "Accepted";

const REJECTED: &str = "Rejected";

/// Launch a webserver, associated with a transaction server, that will
/// answer to workers.
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
                println!("Worker ask for previous transaction !");
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

                // TODO This is not the consensus protocol
                //      I have to change this, somehow.
                // Try to append it to the server
                if server.lock().unwrap().add_block_safe(received) {
                    Response::text(ACCEPTED)
                } else {
                    Response::text(REJECTED)
                }
            },

            _ => Response::empty_404()
        )
    });

}
