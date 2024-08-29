use rblc::blockchain::Blockchain;
use rblc::miner::Miner;
use rblc::transaction::Transaction;

#[test]
fn test_mine_block() {
    let mut blockchain = Blockchain::new();

    // Устанавливаем начальные балансы
    blockchain.set_wallet_balance("user1", 1000);
    blockchain.set_wallet_balance("user2", 0);
    blockchain.set_wallet_balance("miner1", 0); // Баланс майнера на старте

    let miner = Miner::new("miner1".to_string());
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];
    
    miner.mine_block(&mut blockchain, "Test Block".to_string(), transactions.clone());

    // Проверка, что добавлен новый блок
    assert_eq!(blockchain.chain.len(), 2);

    // Проверка, что баланс майнера увеличен
    assert_eq!(blockchain.get_wallet_balance("miner1"), 50);

    // Проверка балансов после транзакции
    assert_eq!(blockchain.get_wallet_balance("user1"), 950); // Предполагается, что начальный баланс был 1000
    assert_eq!(blockchain.get_wallet_balance("user2"), 50);  // Получил 50 от user1
}
