use crate::simple_transaction::SimpleTransaction;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Serialize, Deserialize)]
pub struct Block {
    transactions: SimpleTransaction,
    previous_hash: Option<String>,
    nonce: u64,
    index_in_chain: u64,
    /// Internal representation of the bytes of the transactions
    /// This avoids to recompute it every time that `bytes` is called
    #[serde(skip)]
    immutable_bytes: Vec<u8>
}

impl Block {
    pub fn genesis(transactions: SimpleTransaction) -> Self {
        let bytes: Vec<u8> = transactions.to_bytes();
        Self {
            transactions,
            immutable_bytes: bytes,
            nonce: 0,
            index_in_chain: 0,
            previous_hash: None
        }
    }

    /// Build a new block located after the given block.
    pub fn new_after_block(data: SimpleTransaction, previous: &Block) -> Self {
        let mut block = Self::genesis(data);
        block.previous_hash = Some(previous.hash());
        block.index_in_chain = previous.index_in_chain + 1;
        block
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }

    pub fn set_previous_hash(&mut self, previous_hash: String) {
        self.previous_hash = Some(previous_hash);
    }

    pub fn previous_hash(&self) -> Option<String> {
        self.previous_hash.clone()
    }

    pub fn hash(&self) -> String {
        let data = self.bytes();
        digest(data)
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

    pub fn transactions(&self) -> &SimpleTransaction {
        &self.transactions
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "previous hash = {:?}, nounce = {:?}", self.previous_hash, self.nonce)
    }
}