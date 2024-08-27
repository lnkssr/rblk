use crate::block::Block;
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub wallets: Vec<Wallet>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), vec![]);
        Blockchain {
            chain: vec![genesis_block],
            wallets: Vec::new(),
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain
            .last()
            .expect("Blockchain should have at least one block")
    }

    pub fn add_block(
        &mut self,
        data: String,
        miner_address: String,
        transactions: Vec<Transaction>,
    ) {
        let previous_block = self.get_latest_block();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            transactions,
        );

        self.reward_miner(miner_address, 50);
        self.chain.push(new_block);

        // Save block and wallet data to files
        if let Err(e) = self.save_to_files() {
            eprintln!("Error saving blockchain data: {:?}", e);
        }
    }

    pub fn reward_miner(&mut self, miner_address: String, reward: u64) {
        for wallet in &mut self.wallets {
            if wallet.address == miner_address {
                wallet.balance += reward;
                return;
            }
        }

        let new_wallet = Wallet {
            address: miner_address,
            balance: reward,
        };

        self.wallets.push(new_wallet);

        // Save wallet data to file
        if let Err(e) = self.save_wallets_to_file() {
            eprintln!("Error saving wallet data: {:?}", e);
        }
    }

    pub fn save_to_files(&self) -> Result<(), std::io::Error> {
        let blockchain_path = Path::new("blockchain.json");
        let blockchain_json = serde_json::to_string(self)?;
        fs::write(blockchain_path, blockchain_json)?;

        Ok(())
    }

    pub fn save_wallets_to_file(&self) -> Result<(), std::io::Error> {
        let wallets_path = Path::new("wallets.json");
        let wallets_json = serde_json::to_string(&self.wallets)?;
        fs::write(wallets_path, wallets_json)?;

        Ok(())
    }

    pub fn load_from_files() -> Result<Blockchain, std::io::Error> {
        let blockchain_path = Path::new("blockchain.json");
        let blockchain_json = fs::read_to_string(blockchain_path)?;
        let mut blockchain: Blockchain = serde_json::from_str(&blockchain_json)?;

        let wallets_path = Path::new("wallets.json");
        let wallets_json = fs::read_to_string(wallets_path)?;
        blockchain.wallets = serde_json::from_str(&wallets_json)?;

        Ok(blockchain)
    }

    pub fn get_wallet_balance(&self, address: &str) -> u64 {
        for wallet in &self.wallets {
            if wallet.address == address {
                return wallet.balance;
            }
        }
        0
    }
}
