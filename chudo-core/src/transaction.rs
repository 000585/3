use serde::{Serialize, Deserialize};
use crate::crypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: [u8; 32],
    pub recipient: [u8; 32],
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Option<Vec<u8>>,
    pub hash: [u8; 32],
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(sender: [u8; 32], recipient: [u8; 32], amount: u64, fee: u64, nonce: u64) -> Self {
        Self {
            sender,
            recipient,
            amount,
            fee,
            nonce,
            signature: None,
            hash: [0u8; 32],
            timestamp: 0, 
        }
    }

    pub fn calculate_hash(&mut self) -> [u8; 32] {
        let hash = crypto::hash(self);
        self.hash = hash;
        hash
    }
}
