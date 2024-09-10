use repyh::server::Server;
use repyh::simple_transaction::SimpleTransaction;
use rouille::{router, Response};
use std::sync::{Arc, Mutex};
use repyh::block::Block;

const ACCEPTED: &str = "Accepted";
const REJECTED: &str = "Rejected";

fn handle_web_server(server: Arc<Mutex<Server>>) {
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
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

/// The server is in charge of
/// - keeping track of pending transactions.
/// - responding to clients who want to submit new transactions.
/// - forwarding to workers (nodes) the pending transactions and all the newly mined blocks.
fn main() {
    let mut server = Server::new();

    server.submit_transaction(SimpleTransaction::from_str("Hello"));
    server.submit_transaction(SimpleTransaction::from_str("World"));
    server.submit_transaction(SimpleTransaction::from_str("from arthur"));
    server.submit_transaction(SimpleTransaction::from_str("Again something else"));
    server.submit_transaction(SimpleTransaction::from_str("I got a book on that day"));
    server.submit_transaction(SimpleTransaction::from_str("Please, never forget that i said that one day"));
    server.submit_transaction(SimpleTransaction::from_str("The answer is 42"));

    // Send a thread-safe pointer to the server to the webserver
    let server = Arc::new(Mutex::new(server));

    // Rouille example taken from https://github.com/tomaka/rouille/blob/master/examples/hello-world.rs
    let server_ref = server.clone();
    handle_web_server(server_ref);
}