/// A worker is in charge of constantly mining blocks, and forwarding them to the network
/// It can also be interrupted by other workers, if anyone found before the others a valid block.
pub struct Worker {
    /// List of other workers on the same network
    peers: Vec<String>,
    // Address of the transaction server
    // server: String
}

impl Worker {
    pub fn new() -> Self {
        Self { peers: Vec::new() }
    }


}
