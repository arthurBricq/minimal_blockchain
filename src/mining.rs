use crate::blockchain::Block;
use sha256::digest;

pub fn mine(block: &mut Block) -> String {
    // We are looking for an output that starts with a certain number of zeros
    for nonce in 0..u64::MAX {
        block.set_nonce(nonce);
        let data = block.bytes();
        let hash = digest(data);

        // look for a start with N zeros
        if hash.starts_with("000") {
            return hash
        }
    }

    panic!("")
}
