use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

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
    }

    pub fn save_to_disk(&self) -> std::io::Result<()> {
        let serialized_chain = serde_json::to_string(&self)?;
        let mut file = File::create("blockchain_data.json")?;
        file.write_all(serialized_chain.as_bytes())?;
        Ok(())
    }

    pub fn load_from_disk() -> std::io::Result<Blockchain> {
        let mut file = match File::open("blockchain_data.json") {
            Ok(file) => file,
            Err(_) => {
                println!("Файл не найден. Создаем новый блокчейн с генезис-блоком.");
                return Ok(Blockchain::new());
            }
        };

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        if data.trim().is_empty() {
            println!("Файл пуст. Создаем новый блокчейн с генезис-блоком.");
            return Ok(Blockchain::new());
        }

        let blockchain: Blockchain =
            serde_json::from_str(&data).expect("Ошибка при десериализации данных");
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
