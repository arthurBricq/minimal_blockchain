use std::fmt::{Debug, Formatter};
use crate::transaction::Transaction;

pub struct Block {
    transactions: Vec<Transaction>,
    previous_hash: Option<String>,
    nonce: u64,
    /// Internal representation of the bytes of the transactions
    /// This avoids to recompute it every time that `bytes` is called
    transaction_bytes: Vec<u8>
}

impl Block {
    pub fn new(data: Vec<Transaction>) -> Self {
        let mut bytes: Vec<u8> = data
            .iter()
            .flat_map(|tx| tx.to_bytes())
            .collect();
        Self {
            transactions: data,
            transaction_bytes: bytes,
            nonce: 0,
            previous_hash: None
        }
    }

    /// Returns a bytes representation of this block
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = self.transaction_bytes.clone();
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