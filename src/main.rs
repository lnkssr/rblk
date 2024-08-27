use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u128,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.mine_block(2); // Майним блок с уровнем сложности 2
        block
    }

    fn calculate_hash(&self) -> String {
        let block_content = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.nonce,
            self.transactions_string()
        );
        format!("{:x}", md5::compute(block_content))
    }

    fn transactions_string(&self) -> String {
        let mut transactions_str = String::new();
        for tx in &self.transactions {
            write!(
                &mut transactions_str,
                "{} отправил {} получателю {}; ",
                tx.sender, tx.amount, tx.receiver
            )
            .unwrap();
        }
        transactions_str
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!(
            "Блок с индексом {} замайнен! Хеш: {}",
            self.index, self.hash
        );
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    mining_reward: f64,
}

impl Blockchain {
    fn new(mining_reward: f64) -> Blockchain {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
            mining_reward,
        }
    }

    fn get_latest_block(&self) -> &Block {
        self.chain
            .last()
            .expect("Blockchain should have at least one block")
    }

    fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn mine_pending_transactions(&mut self, miner_address: String) {
        let latest_block_index = self.get_latest_block().index;
    
        // Добавляем награду за майнинг
        let reward_tx = Transaction {
            sender: "System".to_string(),
            receiver: miner_address.clone(),
            amount: self.mining_reward,
        };
    
        self.pending_transactions.push(reward_tx);
    
        // Проверка предыдущего хеша
        let previous_hash = self.get_latest_block().hash.clone();
        if previous_hash.is_empty() {
            panic!("Предыдущий хеш пустой!");
        }
    
        println!("Последний хеш перед майнингом: {}", previous_hash);
    
        let new_block = Block::new(
            latest_block_index + 1,
            self.pending_transactions.clone(),
            previous_hash,
        );
    
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }
    
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new(100.0);

    blockchain.create_transaction(Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 50.0,
    });

    blockchain.create_transaction(Transaction {
        sender: "Bob".to_string(),
        receiver: "Alice".to_string(),
        amount: 25.0,
    });

    blockchain.mine_pending_transactions("Miner1".to_string());

    println!("{:#?}", blockchain);
}
