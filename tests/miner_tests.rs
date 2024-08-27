use rblc::miner::Miner;
use rblc::blockchain::Blockchain;
use rblc::transaction::Transaction;

#[test]
fn test_miner_creation() {
    let miner = Miner::new("miner1".to_string());
    assert_eq!(miner.address, "miner1");
}

#[test]
fn test_mine_block() {
    let mut blockchain = Blockchain::new();
    let miner = Miner::new("miner1".to_string());
    let transactions = vec![
        Transaction::new("user1".to_string(), "user2".to_string(), 50),
    ];
    miner.mine_block(&mut blockchain, "Test Block".to_string(), transactions);
    assert_eq!(blockchain.chain.len(), 2);
}
