use rsa::RsaPublicKey;

/// A transaction submitted on the network
#[derive(Debug)]
pub struct Transaction {
    /// Public key of the sender
    sender_pub_key: RsaPublicKey,
    /// Public key of the receiver
    receiver_pub_key: RsaPublicKey,
    /// Signature
    signature: Vec<u8>,
    amount: u64
}

impl Transaction {
    pub fn new(sender_pub_key: RsaPublicKey, receiver_pub_key: RsaPublicKey, signature: Vec<u8>, amount: u64) -> Self {
        Self { sender_pub_key, receiver_pub_key, signature, amount }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        todo!();
        vec![]
    }
}