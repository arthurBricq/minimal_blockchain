use crate::block::Block;
use sha256::digest;
use tokio_util::sync::CancellationToken;

/// Find the nonce for which the bytes of the given block match a pattern
/// starting with N zeros, where `N` is the `difficulty` argument.
pub async fn mine(
    block: &mut Block,
    difficulty: usize,
    cancellation_token: CancellationToken
) -> Option<String> {
    let start_pattern = String::from_utf8(vec![b'0'; difficulty]).unwrap();

    // We are looking for an output that starts with a certain number of zeros
    for nonce in 0..u64::MAX {
        // look for a start with N zeros
        block.set_nonce(nonce);
        let hash = block.hash();
        if hash.starts_with(&start_pattern) {
            return Some(hash)
        }

        // Always check if this thread was asked to be cancelled
        if cancellation_token.is_cancelled() {
            return None
        }
    }

    None
}

/// Find the nonce for which the bytes of the given block match a pattern
/// starting with N zeros, where `N` is the `difficulty` argument.
pub fn mine_sync(block: &mut Block, difficulty: usize) -> String {

    let start_pattern = String::from_utf8(vec![b'0'; difficulty]).unwrap();

    // We are looking for an output that starts with a certain number of zeros
    for nonce in 0..u64::MAX {
        block.set_nonce(nonce);
        let data = block.bytes();
        let hash = digest(data);

        // look for a start with N zeros
        if hash.starts_with(&start_pattern) {
            return hash
        }
    }

    panic!("")
}
