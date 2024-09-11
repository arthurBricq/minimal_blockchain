use crate::simple_transaction::SimpleTransaction;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    transactions: SimpleTransaction,
    previous_hash: Option<String>,
    nonce: u64,
    index_in_chain: u64,
}

impl Block {
    /// Creates the original block
    /// This is a block with no data and with a nonce computed for a difficulty of 5 zeros.
    pub fn genesis() -> Self {
        Self {
            transactions: SimpleTransaction::from_str(""),
            // This nonce was generated for a difficulty of 5 zeros
            nonce: 1293653,
            index_in_chain: 0,
            previous_hash: None
        }
    }

    /// Build a new block located after the given block.
    pub fn new_after_block(data: SimpleTransaction, previous: &Block) -> Self {
        Self {
            transactions: data,
            previous_hash: Some(previous.hash()),
            nonce: 0,
            index_in_chain: previous.index_in_chain + 1
        }
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
    
    pub fn is_hash_valid(&self, difficulty: usize) -> bool {
        let start_pattern = String::from_utf8(vec![b'0'; difficulty]).unwrap();
        self.hash().starts_with(&start_pattern)
    }

    /// Returns a bytes representation of this block
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.transactions.to_bytes();
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        if let Some(hash) = &self.previous_hash {
            bytes.extend_from_slice(hash.as_bytes());
        }
        bytes
    }

    pub fn transactions(&self) -> &SimpleTransaction {
        &self.transactions
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
    
    pub fn print_block(&self) {
        log::info!("  * nonce    = {}", self.nonce());
        log::info!("  * previous = {:?}", self.previous_hash().unwrap());
        log::info!("  * hash     = {}", self.hash());
        log::info!("  * data     = {:?}", self.transactions);
    }

    pub fn index_in_chain(&self) -> u64 {
        self.index_in_chain
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "previous hash = {:?}, nounce = {:?}", self.previous_hash, self.nonce)
    }
}

#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::simple_transaction::SimpleTransaction;

    #[test]
    fn test_hash_consistency()  {
        let mut b1 = Block::genesis();
        b1.set_nonce(1234);
        let as_json = serde_json::to_string(&b1).unwrap();
        let b1_parsed: Block = serde_json::from_str(&as_json).unwrap();
        assert_eq!(b1.hash(), b1_parsed.hash());
        
        let mut b2 = Block::new_after_block(SimpleTransaction::from_str("Hello world"), &b1);
        b2.set_nonce(9876);
        let as_json = serde_json::to_string(&b2).unwrap();
        let b2_parsed: Block = serde_json::from_str(&as_json).unwrap();
        assert_eq!(b2.hash(), b2_parsed.hash());
    }
}