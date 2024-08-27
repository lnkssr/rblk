use rblc::block::Block;
use rblc::transaction::Transaction;

#[test]
fn test_block_creation() {
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];
    let block = Block::new(1, "Test Block".to_string(), "0".to_string(), transactions);
    assert_eq!(block.index, 1);
    assert_eq!(block.data, "Test Block");
    assert_eq!(block.previous_hash, "0");
    assert!(!block.hash.is_empty());
}

#[test]
fn test_block_mining() {
    let transactions = vec![Transaction::new(
        "user1".to_string(),
        "user2".to_string(),
        50,
    )];
    let mut block = Block::new(1, "Test Block".to_string(), "0".to_string(), transactions);
    block.mine_block();
    assert!(block.hash.starts_with("0000"));
}
