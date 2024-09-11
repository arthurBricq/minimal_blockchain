# A mock Block Chain in Rust

A simple blockchain to store **text information** immutably across the network.

- Transactions are just plain text: they are not signed, nor do they keep track of whom sent them.

- Workers communicate between themselves on a P2P network (`libp2p.rs`) when they finish to mine. This allows other workers to abort mining if they found the block to be valid.

- There is a centralized web server (`server.rs`) which acts as the **mempool** of the network. Each node request the server a new transaction through a `GET` request.

# Getting started

1. Run the server to dispatch transactions

```console
cargo run --bin server
```

Note that the server is instantiated with a bunch of initial transactions (see `main_server.rs`), but you can yourself add a transaction to be saved on the chain.

```console
cargo run --bin submit -- "All that is gold does not glitter, Not all those who wander are lost; The old that is strong does not wither, Deep roots are not reached by the frost."
```

2. Run one (or many) workers, each in his own terminal

```console
cargo run --bin node
```

# Remaining work 

List of remaining challenges

1. Blockchain divergence: when two workers finish to mine at roughly the same time, they will both have the time to dispatch their node and they will both 
2. Removing pending transactions from the server
3. Downloading the chain from other workers when a new worker connects.

# Resources

- Blog post about async await https://ryhl.io/blog/async-what-is-blocking/
- Book about async await https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
- Tokio documentation https://tokio.rs/tokio/tutorial/spawning
- libp2p.rs documentation (and mostly the `chat` example) https://github.com/libp2p/rust-libp2p/blob/master/examples/chat/src/main.rs
