use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    transactions: Vec<Transaction>,
    previous_hash: Option<String>,
    nonce: u64,
}

impl Block {
    pub fn new(data: Vec<Transaction>) -> Self {
        Self {
            transactions: data,
            nonce: 0,
            previous_hash: None
        }
    }

    /// Returns a bytes representation of this block
    pub fn bytes(&self) ->Vec<u8> {
        let mut bytes: Vec<u8> = self.transactions
            .iter()
            .flat_map(|tx| tx.to_bytes())
            .collect();
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
