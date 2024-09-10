use std::sync::{Arc, Mutex};
use rouille::{router, Response};
use repyh::client::Client;
use repyh::server::Server;

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
                server.lock().unwrap().register_worker(port);
                println!("Current workers: {:?}", server.lock().unwrap().workers());
                Response::text("registered")
            },
            
            (GET) (/get_transactions/{last: u32}) => {
                // Workers ask for a list of all the other workers
                println!("Worker ask for previous {last} transactions !");
                Response::text("TODO")
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
    let mut client1 = Client::new(100);
    server.submit_transaction(client1.emit_transaction(client1.public_key(), 5).unwrap());
    server.submit_transaction(client1.emit_transaction(client1.public_key(), 5).unwrap());

    // Send a thread-safe pointer to the server to the webserver
    let server = Arc::new(Mutex::new(server));

    // Rouille example taken from https://github.com/tomaka/rouille/blob/master/examples/hello-world.rs
    let server_ref = server.clone();
    handle_web_server(server_ref);
}