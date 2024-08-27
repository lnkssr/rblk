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
    blockchain.reward_miner("miner1".to_string(), 50);
    assert_eq!(blockchain.get_wallet_balance("miner1"), 50);
}
