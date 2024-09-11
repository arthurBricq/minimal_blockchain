# A mock Block Chain in Rust

A simple blockchain to store **text information** immutably across the network.

- **Transactions** are just plain text: they are not signed, nor do they keep track of whom sent them. They can be written exactly once on the blockchain.

- Workers communicate between themselves on a P2P network (`libp2p.rs`) when they finish to mine. This allows other workers to abort mining if they found the block to be valid.

- Workers are able to track blockchain divergence across time, by basically keep track of several chains. AS soon as one becomes longer than the others, this chain becomes the new 'main' chain.

- There is a centralized web server (`server.rs`) which acts as the **mempool** of the network. Each node request the server a new transaction through a `GET` request.

- Transaction are deleted server from the mempool when the message is written deep enough in the blockchain representation of the server.

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

1. Blockchain divergence usually takes a few iteration to be resolved.

2. Downloading the chain from other workers when a new worker connects.

3. Even though it is unit-tested, there seems to be problems with keeping track of the divergence

# Resources

- Blog post about async await https://ryhl.io/blog/async-what-is-blocking/
- Book about async await https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
- Tokio documentation https://tokio.rs/tokio/tutorial/spawning
- libp2p.rs documentation (and mostly the `chat` example) https://github.com/libp2p/rust-libp2p/blob/master/examples/chat/src/main.rs
