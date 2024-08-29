use rblc::blockchain::Blockchain;
use rblc::transaction::Transaction;

#[test]
fn test_create_new_blockchain() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.chain.len(), 1);
    assert!(blockchain.chain[0].data.contains("Genesis Block"));
}

#[test]
fn test_add_block_to_blockchain() {
    let mut blockchain = Blockchain::new();
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];
    blockchain.add_block("New Block".to_string(), "miner1".to_string(), transactions);
    assert_eq!(blockchain.chain.len(), 2);
}

#[test]
fn test_reward_miner() {
    let mut blockchain = Blockchain::new();

    // Награждаем майнера через добавление блока
    blockchain.add_block("New Block".to_string(), "miner1".to_string(), vec![]);

    // Проверяем, что баланс майнера увеличился на 50 токенов
    assert_eq!(blockchain.get_wallet_balance("miner1"), 50);
}

#[test]
fn test_set_wallet_balance() {
    let mut blockchain = Blockchain::new();
    blockchain.set_wallet_balance("user1", 1000);
    assert_eq!(blockchain.get_wallet_balance("user1"), 1000);
}

#[test]
fn test_transaction_balances() {
    let mut blockchain = Blockchain::new();

    // Устанавливаем начальный баланс для user1
    blockchain.set_wallet_balance("user1", 1000);

    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        200,
    )];
    blockchain.add_block(
        "Transaction Block".to_string(),
        "miner1".to_string(),
        transactions,
    );

    // Проверка балансов после транзакции
    assert_eq!(blockchain.get_wallet_balance("user1"), 800); // 1000 - 200
    assert_eq!(blockchain.get_wallet_balance("user2"), 200); // +200
}

#[test]
fn test_valid_chain() {
    let mut blockchain = Blockchain::new();
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];

    // Добавляем несколько блоков
    blockchain.add_block(
        "First Block".to_string(),
        "miner1".to_string(),
        transactions.clone(),
    );
    blockchain.add_block(
        "Second Block".to_string(),
        "miner1".to_string(),
        transactions,
    );

    // Проверяем валидность цепочки
    assert!(blockchain.is_chain_valid(), "Цепочка должна быть валидной.");
}

#[test]
fn test_invalid_chain_due_to_modified_block() {
    let mut blockchain = Blockchain::new();
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];

    // Добавляем несколько блоков
    blockchain.add_block(
        "First Block".to_string(),
        "miner1".to_string(),
        transactions.clone(),
    );
    blockchain.add_block(
        "Second Block".to_string(),
        "miner1".to_string(),
        transactions,
    );

    // Модифицируем данные блока (подделка)
    blockchain.chain[1].data = "Tampered Data".to_string();
    blockchain.chain[1].hash = blockchain.chain[1].calculate_hash(); // Обновляем хеш для согласования

    // Проверяем валидность цепочки
    assert!(
        !blockchain.is_chain_valid(),
        "Цепочка должна быть невалидной после модификации."
    );
}

#[test]
fn test_invalid_chain_due_to_wrong_previous_hash() {
    let mut blockchain = Blockchain::new();
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];

    // Добавляем несколько блоков
    blockchain.add_block(
        "First Block".to_string(),
        "miner1".to_string(),
        transactions.clone(),
    );
    blockchain.add_block(
        "Second Block".to_string(),
        "miner1".to_string(),
        transactions,
    );

    // Модифицируем `previous_hash` в одном из блоков
    blockchain.chain[2].previous_hash = "InvalidPreviousHash".to_string();

    // Проверяем валидность цепочки
    assert!(
        !blockchain.is_chain_valid(),
        "Цепочка должна быть невалидной из-за неверного previous_hash."
    );
}
