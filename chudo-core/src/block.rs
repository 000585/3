use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use bincode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub difficulty: u32,
    pub nonce: u64,
    pub miner: [u8; 32],
    pub height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: [u8; 32],
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        let mut block = Self {
            header,
            transactions,
            hash: [0u8; 32],
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> [u8; 32] {
        // ??????????? ????????? ? ?????
        let data = bincode::serialize(&self.header).expect("Serialization failed");
        
        // ?????????? ??? ????? kHeavyHash
        let hash_vec = crate::crypto::k_heavy_hash(&data);

        let mut hash = [0u8; 32];
        if hash_vec.len() >= 32 {
            hash.copy_from_slice(&hash_vec[0..32]);
        }
        hash
    }
}
