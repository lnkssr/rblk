mod block;
mod blockchain;
mod miner;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use miner::Miner;
use transaction::Transaction;
use std::process;
fn main() {
    // Загрузка блокчейна из файлов
    let mut blockchain = Blockchain::load_from_files().unwrap_or_else(|_| Blockchain::new());

    if blockchain.is_chain_valid() {
        println!("Blockchain is valid");
    } else {
        println!("Blockchain invalid");
        process::exit(1);
    }

    // Назначаем начальные балансы для пользователей
    blockchain.set_wallet_balance("user1", 1000);
    blockchain.set_wallet_balance("user2", 500);
    blockchain.set_wallet_balance("user3", 1000);

    // Создание майнера
    let miner = Miner::new("miner1".to_string());

    // Пример транзакций
    let transactions = vec![
        Transaction::new("user1".to_string(), "user2".to_string(), 50),
        Transaction::new("user2".to_string(), "user3".to_string(), 25),
        Transaction::new("user3".to_string(), "user1".to_string(), 250),
    ];

    // Майнинг новых блоков
    miner.mine_block(
        &mut blockchain,
        "First block after genesis".to_string(),
        transactions.clone(),
    );
    miner.mine_block(
        &mut blockchain,
        "Second block after genesis".to_string(),
        transactions.clone(),
    );

    // Сохранение блокчейна и кошельков в файлы
    blockchain
        .save_to_files()
        .expect("Error saving blockchain data");
    blockchain
        .save_wallets_to_file()
        .expect("Error saving wallet data");

    // Печать блоков
    for block in blockchain.chain.iter() {
        println!(
            "Block {{ index: {}, timestamp: {}, data: '{}', previous_hash: '{}', hash: '{}', nonce: {}, transactions: {:?} }}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash, block.nonce, block.transactions
        );
    }

    // Печать баланса майнера
    println!(
        "Miner wallet balance {}: {} tokens",
        miner.address,
        blockchain.get_wallet_balance(&miner.address)
    );

    // Печать балансов пользователей
    println!("User1 balance: {}", blockchain.get_wallet_balance("user1"));
    println!("User2 balance: {}", blockchain.get_wallet_balance("user2"));
    println!("User3 balance: {}", blockchain.get_wallet_balance("user3"));
}
