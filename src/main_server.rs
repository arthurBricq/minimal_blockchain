use repyh::simple_transaction::SimpleTransaction;
use std::sync::{Arc, Mutex};
use env_logger::Env;
use crate::server::{run_web_server, Server};

mod server;

/// The server is in charge of
/// - keeping track of pending transactions.
/// - responding to clients who want to submit new transactions.
fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    
    let mut server = Server::new();

    // Submit a bunch of transaction
    server.submit_transaction(SimpleTransaction::from_str("Hello World"));
    server.submit_transaction(SimpleTransaction::from_str("Starship landed"));
    server.submit_transaction(SimpleTransaction::from_str("He dies on mars"));
    server.submit_transaction(SimpleTransaction::from_str("Again something else"));
    server.submit_transaction(SimpleTransaction::from_str("I got a book on that day"));
    server.submit_transaction(SimpleTransaction::from_str("Please, never forget that i said that one day"));
    server.submit_transaction(SimpleTransaction::from_str("The answer is 42"));
    server.submit_transaction(SimpleTransaction::from_str("In the beginning the Universe was created"));
    server.submit_transaction(SimpleTransaction::from_str("This was a bad move"));
    server.submit_transaction(SimpleTransaction::from_str("He was his father"));
    server.submit_transaction(SimpleTransaction::from_str("One chain to rule them all"));

    // Send a thread-safe pointer to the server to the webserver
    let server = Arc::new(Mutex::new(server));

    // Rouille example taken from https://github.com/tomaka/rouille/blob/master/examples/hello-world.rs
    let server_ref = server.clone();
    run_web_server(server_ref);
}