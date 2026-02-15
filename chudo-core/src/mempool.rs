use std::collections::HashMap;
use crate::transaction::Transaction;

pub struct Mempool {
    transactions: HashMap<String, Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }

    pub fn add(&mut self, tx_hash: String, tx: Transaction) -> Result<(), String> {
        if self.transactions.contains_key(&tx_hash) {
            return Err("Transaction already in mempool".to_string());
        }
        self.transactions.insert(tx_hash, tx);
        Ok(())
    }

    pub fn get(&self, tx_hash: &str) -> Option<&Transaction> {
        self.transactions.get(tx_hash)
    }

    pub fn remove(&mut self, tx_hash: &str) -> Option<Transaction> {
        self.transactions.remove(tx_hash)
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn get_all(&self) -> Vec<(String, Transaction)> {
        self.transactions
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
    }
}
