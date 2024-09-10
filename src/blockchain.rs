pub struct BlockChain {
    genesis: Block
}


#[derive(Debug)]
pub struct Block {
    data: u64,
    nonce: u64,
    previous_hash: Option<String>
}

impl Block {
    pub fn new(data: u64) -> Self {
        Self {
            data,
            nonce: 0,
            previous_hash: None
        }
    }

    pub fn bytes(&self) ->Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.data.to_le_bytes());
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

impl BlockChain {
    pub fn new(genesis: Block) -> Self {
        Self { genesis }
    }

}

