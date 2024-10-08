use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};

/// A very simple transaction that can be shared on the network
/// The goal of saving this transaction on the blockchain is to have its record written immutably across
/// many computers.
#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct SimpleTransaction {
    message: String,
}

impl SimpleTransaction {
    pub fn new() -> Self {
        Self {
            message: Default::default(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.message.as_bytes().to_vec()
    }

    pub fn from_str(text: &str) -> Self {
        Self {
            message: text.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}

impl Debug for SimpleTransaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.message)
    }
}
