use serde::{Deserialize, Serialize};
use std::str::from_utf8;

/// A very simple transaction that can be shared on the network
/// The goal of saving this transaction on the blockchain is to have its record written immutably across
/// many computers.
#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleTransaction {
    data: Vec<u8>,
}

impl SimpleTransaction {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn from_str(text: &str) -> Self {
        Self {
            data: text.as_bytes().to_vec(),
        }
    }

    pub fn to_string(&self) -> String {
        from_utf8(&self.data).unwrap().to_string()
    }
}
