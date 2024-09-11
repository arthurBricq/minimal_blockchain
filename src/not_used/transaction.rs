use rsa::pkcs1v15::Signature;
use rsa::RsaPublicKey;
use rsa::signature::SignatureEncoding;
use crate::client::public_key_to_bytes;

/// A transaction submitted on the network
#[derive(Debug)]
pub struct Transaction {
    /// Public key of the sender
    sender_pub_key: RsaPublicKey,
    /// Public key of the receiver
    receiver_pub_key: RsaPublicKey,
    /// Signature
    signature: Signature,
    /// Amount being transferred
    amount: u64
}

impl Transaction {
    pub fn new(
        sender_pub_key: RsaPublicKey,
        receiver_pub_key: RsaPublicKey,
        signature: Signature,
        amount: u64
    ) -> Self {
        Self {
            sender_pub_key,
            receiver_pub_key,
            signature,
            amount
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let part1 = public_key_to_bytes(&self.sender_pub_key);
        let part2 = public_key_to_bytes(&self.receiver_pub_key);
        let part3 = Vec::from(self.amount.to_be_bytes());
        let part4 = self.signature.to_bytes().to_vec();
        [part1, part2, part3, part4].concat()
    }
}
