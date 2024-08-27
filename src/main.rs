mod block;
mod blockchain;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    // Загружаем блокчейн из файла
    let mut blockchain = Blockchain::load_from_disk().unwrap_or_else(|_| Blockchain::new());

    // Адрес майнера
    let miner_address = "miner1".to_string();

    // Пример транзакций
    let transactions = vec![
        Transaction::new("user1".to_string(), "user2".to_string(), 50),
        Transaction::new("user2".to_string(), "user3".to_string(), 25),
    ];

    // Майним новые блоки
    blockchain.add_block(
        "Первый блок после генезиса".to_string(),
        miner_address.clone(),
        transactions.clone(),
    );
    blockchain.add_block(
        "Второй блок после генезиса".to_string(),
        miner_address.clone(),
        transactions.clone(),
    );

    // Сохраняем блокчейн на диск
    blockchain
        .save_to_disk()
        .expect("Ошибка при сохранении блокчейна");

    // Выводим блоки
    for block in blockchain.chain.iter() {
        println!(
            "Block {{ index: {}, timestamp: {}, data: '{}', previous_hash: '{}', hash: '{}', nonce: {}, transactions: {:?} }}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash, block.nonce, block.transactions
        );
    }

    // Выводим баланс майнера
    println!(
        "Баланс кошелька майнера {}: {} токенов",
        miner_address,
        blockchain.get_wallet_balance(&miner_address)
    );
}
