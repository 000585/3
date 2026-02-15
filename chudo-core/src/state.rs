use std::collections::HashMap;
use crate::block::{Block, BlockHeader};
use crate::transaction::Transaction;
use crate::crypto;
// Убрали ошибочный use crate::dag::Dag;

// Структура должна быть PUB
pub struct BlockchainState {
    pub blocks: HashMap<[u8; 32], Block>,
    pub balances: HashMap<[u8; 32], u64>,
    pub mempool: Vec<Transaction>,
    pub height: u64,
    pub last_hash: [u8; 32],
}

impl BlockchainState {
    pub fn new() -> Self {
        let genesis_hash = [0u8; 32]; 
        
        Self {
            blocks: HashMap::new(),
            balances: HashMap::new(),
            mempool: Vec::new(),
            height: 0,
            last_hash: genesis_hash,
        }
    }

    pub fn get_balance(&self, address: &[u8; 32]) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        let sender_balance = self.get_balance(&tx.sender);
        if sender_balance < tx.amount + tx.fee {
            return Err("Insufficient funds".to_string());
        }
        self.mempool.push(tx);
        Ok(())
    }

    // Исправили Vec<u8> на [u8; 32], чтобы совпадало с Node
    pub fn mine_block(&mut self, miner: [u8; 32]) -> Block {
        let transactions = self.mempool.drain(..).collect::<Vec<_>>();

        let header = BlockHeader {
            version: 1,
            prev_hash: self.last_hash,
            merkle_root: [0u8; 32],
            timestamp: 123456789,
            difficulty: 1,
            nonce: 0,
            miner, // Теперь типы совпадают
            height: self.height + 1,
        };

        let mut block = Block::new(header, transactions);
        block.hash = crypto::hash(&block);

        self.apply_block(&block);

        block
    }

    fn apply_block(&mut self, block: &Block) {
        let miner_balance = self.balances.entry(block.header.miner).or_insert(0);
        *miner_balance += 50;

        for tx in &block.transactions {
            let sender_bal = self.balances.entry(tx.sender).or_insert(0);
            if *sender_bal >= tx.amount + tx.fee {
                *sender_bal -= tx.amount + tx.fee;
            }

            let recipient_bal = self.balances.entry(tx.recipient).or_insert(0);
            *recipient_bal += tx.amount;
        }

        self.blocks.insert(block.hash, block.clone());
        self.last_hash = block.hash;
        self.height = block.header.height;
    }

    pub fn current_height(&self) -> u64 {
        self.height
    }

    pub fn get_block(&self, hash: &[u8; 32]) -> Option<&Block> {
        self.blocks.get(hash)
    }
    
    pub fn total_supply(&self) -> u64 {
        self.balances.values().sum()
    }
}
