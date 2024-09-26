use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use std::fs::File;
use std::io::{self, Write};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub wallets: Vec<Wallet>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), vec![]);
        Self {
            chain: vec![genesis_block],
            wallets: Vec::new(),
        }
    }

    pub fn add_block(&mut self, data: String, miner_address: String) {
        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone(), vec![]);
        new_block.mine_block();
        self.chain.push(new_block);
        self.update_wallets(miner_address);
        self.save_to_files().expect("Error saving blockchain");
    }

    pub fn execute_transaction(&mut self, transaction: &Transaction) -> Result<(), String> {
        let sender_index = self.get_wallet(&transaction.from).ok_or("Sender wallet not found")?;
        let receiver_index = self.get_wallet(&transaction.to).ok_or("Receiver wallet not found")?;
    
        // Получаем баланс и изменяем его, избегая конфликта заимствований
        let sender_balance = self.wallets[sender_index].get_balance();
        let receiver_balance = self.wallets[receiver_index].get_balance();
    
        if sender_balance < transaction.amount {
            return Err("Insufficient funds".to_string());
        }
    
        self.wallets[sender_index].set_balance(sender_balance - transaction.amount);
        self.wallets[receiver_index].set_balance(receiver_balance + transaction.amount);
        Ok(())
    }
    

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain must contain at least one block")
    }

    pub fn save_to_files(&self) -> io::Result<()> {
        let mut chain_file = File::create("blockchain.json")?;
        let mut wallets_file = File::create("wallets.json")?;
        serde_json::to_writer(&mut chain_file, &self.chain)?;
        serde_json::to_writer(&mut wallets_file, &self.wallets)?;
        Ok(())
    }

    pub fn get_wallet(&self, address: &String) -> Option<usize> {
        self.wallets.iter().position(|wallet| &wallet.address == address)
    }

    pub fn is_chain_valid(&self) -> bool {
        self.chain.iter().zip(self.chain.iter().skip(1)).all(|(prev, curr)| {
            curr.index == prev.index + 1 && curr.previous_hash == prev.hash
        })
    }

    fn update_wallets(&mut self, miner_address: String) {
        if let Some(miner_index) = self.get_wallet(&miner_address) {
            let miner_balance = self.wallets[miner_index].get_balance();
            self.wallets[miner_index].set_balance(miner_balance + 50); // Block reward
        }
    }    
}
