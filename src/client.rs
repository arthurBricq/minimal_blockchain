use std::fmt::{Display, Formatter};
use rand::prelude::ThreadRng;
use rsa::{Oaep, Pkcs1v15Encrypt, Pkcs1v15Sign, Pss, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, Signer};
use rsa::traits::PublicKeyParts;
use crate::transaction::Transaction;

/// Returns the bytes representation of a public key
pub fn public_key_to_bytes<K: PublicKeyParts>(key: &K) -> Vec<u8> {
    let n_bytes = key.n().to_bytes_be(); // Big-endian byte array of modulus
    let e_bytes = key.e().to_bytes_be(); // Big-endian byte array of exponent
    [n_bytes, e_bytes].concat()
}

/// The client to the blockchain network
/// Contains the wallet of the user.
pub struct Client {
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
    amount: u64,
    /// Random number generator
    rng: ThreadRng
}

impl Client {
    /// Creates a new client
    pub fn new(initial_amount: u64) -> Self {
        // Taken from https://docs.rs/rsa/0.9.6/rsa/#pkcs1-v15-encryption
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        Self {
            public_key,
            private_key,
            amount: initial_amount,
            rng,
        }
    }

    /// Creates a transaction to another client of the network
    ///
    /// Returns None if client does not have enough money.
    pub fn emit_transaction(&mut self, receiver: RsaPublicKey, amount: u64) -> Result<Transaction, Box<dyn std::error::Error>> {
        // Check that the client can emit the message
        // Note that the amount is not updated `yet`...
        // The transaction first has to be mined
        if self.amount < amount {
            return Err(Box::new(TransactionError::NotEnoughSold));
        }

        // build the message
        let part1 = public_key_to_bytes(&self.public_key);
        let part2 = public_key_to_bytes(&receiver);
        let part3 = amount.to_be_bytes();
        let data = [part1, part2, Vec::from(part3)].concat();

        // sign the message
        let signing_key = SigningKey::<Sha256>::new(self.private_key.clone());
        let signature = signing_key.sign_with_rng(&mut self.rng, &data);

        // Message is constructed
        Ok(Transaction::new(self.public_key.clone(), receiver, signature, amount))
    }

    pub fn public_key(&self) -> RsaPublicKey {
        self.public_key.clone()
    }
}

#[derive(Debug)]
pub enum TransactionError {
    NotEnoughSold
}

impl std::error::Error for TransactionError {}

impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::NotEnoughSold => write!(f, "Not enough sold")
        }
    }
}
