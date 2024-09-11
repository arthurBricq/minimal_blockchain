# A mock Block Chain in Rust

A simple blockchain to store **text information** immutably across the network.

- **Transactions** are just plain text: they are not signed, nor do they keep track of whom sent them. They can be written exactly once on the blockchain.

- Workers work on a **proof-of-work** by trying to create a **block** containing (1) a single transaction, (2) a nonce and (3) a hash to a previous block which starts with an abritrary pattern of '0's.

- Workers communicate between themselves on a **P2P network** (`libp2p.rs`) to communicate to their peers when they finish to mine. This allows other workers to abort mining if they found the block to be valid.

- Workers are able to track and solve **blockchain divergence** across time, by basically recording several chains: as soon as one becomes longer than the main chain, it becomes the new 'main' chain. Forks are dropped when they become too far from the head of the chain.

- There is a centralized web server (`server.rs`) which acts as the **mempool** of the network. Each worker request the server a new transaction through a `GET` request. If the worker already mined this transaction, then he asks for another one (until that the transaction is flagged as 'safe' by the server and removed from the mempool)

- Transaction are deleted server from the mempool when the message is written deep enough in the blockchain representation of the server.

- **Orphan** blocks (*blocks received but not attached to any chain*) are stored and eventually placed.

# Getting started

1. Run the server to dispatch transactions

```console
cargo run --bin server
```

Note that the server is instantiated with a bunch of initial transactions (see `main_server.rs`), but you can yourself add a transaction to be saved on the chain.

```console
cargo run --bin submit -- "All that is gold does not glitter, Not all those who wander are lost; The old that is strong does not wither, Deep roots are not reached by the frost."
```

2. Run one (or many) workers, each in his own terminal. Each worker will wait about 20 seconds before starting to mine, to make sure that other peers detected him.

```console
cargo run --bin node
```

Due to a current limitation (see below), you have to  **start all you workers in the first 10 seconds after that you launch the first worker**.

# Current limitations

List of remaining problems that I am aware of

- Blockchain divergence usually takes a few iteration to be resolved.

- Even though it is unit-tested, there are still situations in which the divergence does not resolve.

- Orphan blocks : on my machine they are very common. The basic behavior is also unit-tested, but also here they are not well handled and sometimes you end up with an orphan block that sticks forever.

- Downloading the chain from other workers when a new worker connects. I still haven't implemented it, spending too much time debugging the divergence issue. **So when you run the setup, you have to start all you workers in the first 10 seconds**.

# Resources

- Blog post about async await https://ryhl.io/blog/async-what-is-blocking/
- Book about async await https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
- Tokio documentation https://tokio.rs/tokio/tutorial/spawning
- libp2p.rs documentation (and mostly the `chat` example) https://github.com/libp2p/rust-libp2p/blob/master/examples/chat/src/main.rs
- bitcoin white paper
