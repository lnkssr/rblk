use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
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

    pub fn get_wallet(&mut self, address: &str) -> usize {
        if let Some(index) = self.wallets.iter().position(|wallet| wallet.address == address) {
            return index;
        }

        let new_wallet = Wallet::new();
        self.wallets.push(new_wallet);
        self.wallets.len() - 1
    }

    pub fn add_block(&mut self, data: String, miner_address: String, transactions: Vec<Transaction>) {
        let previous_block = self.get_latest_block();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            transactions.clone(),
        );

        for transaction in transactions {
            let sender_index = self.get_wallet(&transaction.from);
            let receiver_index = self.get_wallet(&transaction.to);

            let (wallets_left, wallets_right) = if sender_index <= receiver_index {
                self.wallets.split_at_mut(receiver_index + 1)
            } else {
                self.wallets.split_at_mut(sender_index + 1)
            };

            let sender_wallet = if sender_index <= receiver_index {
                &mut wallets_left[sender_index]
            } else {
                &mut wallets_right[receiver_index]
            };

            if sender_wallet.get_balance() >= transaction.amount {
                sender_wallet.set_balance(sender_wallet.get_balance() - transaction.amount);
            // inst work, i dont know why)  receiver_wallet.set_balance(receiver_wallet.get_balance() + transaction.amount);
            } else {
                println!("Ошибка: Недостаточно средств на кошельке отправителя");
            }
        }

        let miner_index = self.get_wallet(&miner_address);
        let miner_wallet = &mut self.wallets[miner_index];
        miner_wallet.set_balance(miner_wallet.get_balance() + 50); // Награда майнеру

        self.chain.push(new_block);

        if let Err(e) = self.save_to_files() {
            eprintln!("Ошибка сохранения данных блокчейна: {:?}", e);
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain
            .last()
            .expect("Blockchain should have at least one block")
    }

    pub fn save_to_files(&self) -> Result<(), std::io::Error> {
        let blockchain_path = Path::new("blockchain.json");
        let blockchain_json = serde_json::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Ошибка сериализации: {}", e)))?;
        fs::write(blockchain_path, blockchain_json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Ошибка записи в файл: {}", e)))?;
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

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                println!(
                    "Ошибка: Хеш блока с индексом {} неверен.",
                    current_block.index
                );
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                println!(
                    "Ошибка: Хеш предыдущего блока с индексом {} неверен.",
                    current_block.index
                );
                return false;
            }
        }
        true
    }
}
