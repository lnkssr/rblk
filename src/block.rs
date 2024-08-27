use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

const DIFFICULTY_PREFIX: &str = "0000";

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}
impl Block {
    pub fn new(
        index: u64,
        data: String,
        previous_hash: String,
        transactions: Vec<Transaction>,
    ) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            transactions,
        };

        block.mine_block();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let transactions_str = serde_json::to_string(&self.transactions).unwrap();
        let block_content = format!(
            "{}{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce, transactions_str
        );
        let mut hasher = Sha256::new();
        hasher.update(block_content);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine_block(&mut self) {
        while !self.hash.starts_with(DIFFICULTY_PREFIX) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}
