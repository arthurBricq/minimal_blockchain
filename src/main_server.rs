use repyh::server::Server;
use repyh::simple_transaction::SimpleTransaction;
use rouille::{router, Response};
use std::sync::{Arc, Mutex};

fn handle_web_server(server: Arc<Mutex<Server>>) {
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                // If the request's URL is `/`, we jump here.
                Response::redirect_302("/hello/world")
            },

            (GET) (/hello/world) => {
                println!("hello world");
                Response::text("hello world from server")
            },

            (GET) (/register_worker/{port: String}) => {
                // Workers ask for a list of all the other workers
                println!("Worker ask for registration: {port}");
                Response::text("registered")
            },
            
            (GET) (/get_transaction) => {
                // Workers ask for a list of all the other workers
                println!("Worker ask for previous transaction !");
                if let Some(transaction) = server.lock().unwrap().get_last_transaction() {
                    let as_json = serde_json::to_string(&transaction).unwrap();
                    Response::text(as_json)
                } else {
                    Response::text("")
                }
            },

            (GET) (/{id: u32}) => {
                println!("u32 {:?}", id);
                Response::empty_400()
            },

            (GET) (/{id: String}) => {
                println!("String {:?}", id);
                Response::text(format!("hello, {}", id))
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

    // Dummy fill of the server (for now)
    // let mut client1 = Client::new(100);
    // server.submit_transaction(client1.emit_transaction(client1.public_key(), 5).unwrap());
    // server.submit_transaction(client1.emit_transaction(client1.public_key(), 5).unwrap());
    
    server.submit_transaction(SimpleTransaction::from_str("Hello"));
    server.submit_transaction(SimpleTransaction::from_str("World"));
    server.submit_transaction(SimpleTransaction::from_str("from arthur"));

    // Send a thread-safe pointer to the server to the webserver
    let server = Arc::new(Mutex::new(server));

    // Rouille example taken from https://github.com/tomaka/rouille/blob/master/examples/hello-world.rs
    let server_ref = server.clone();
    handle_web_server(server_ref);
}