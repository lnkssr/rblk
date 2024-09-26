use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use chrono;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let mut block = Block {
            index,
            timestamp: chrono::Utc::now().timestamp_millis() as u128,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self) {
        let difficulty = 4;
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block {} mined!", self.index);
    }
}
