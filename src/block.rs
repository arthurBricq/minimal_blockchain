use crate::simple_transaction::SimpleTransaction;
use std::fmt::{Debug, Formatter};

pub struct Block {
    transactions: Vec<SimpleTransaction>,
    previous_hash: Option<String>,
    nonce: u64,
    /// Internal representation of the bytes of the transactions
    /// This avoids to recompute it every time that `bytes` is called
    immutable_bytes: Vec<u8>
}

impl Block {
    pub fn new(transactions: Vec<SimpleTransaction>) -> Self {
        let bytes: Vec<u8> = transactions
            .iter()
            .flat_map(|tx| tx.to_bytes())
            .collect();
        Self {
            transactions,
            immutable_bytes: bytes,
            nonce: 0,
            previous_hash: None
        }
    }

    pub fn new_from_hash(data: Vec<SimpleTransaction>, previous_hash: String) -> Self {
        let mut block = Self::new(data);
        block.set_previous_hash(previous_hash);
        block
    }

    /// Returns a bytes representation of this block
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = self.immutable_bytes.clone();
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        if let Some(hash) = &self.previous_hash {
            bytes.extend_from_slice(hash.as_bytes());
        }
        bytes
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }

    pub fn set_previous_hash(&mut self, previous_hash: String) {
        self.previous_hash = Some(previous_hash);
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "previous hash = {:?}, nounce = {:?}", self.previous_hash, self.nonce)
    }
}