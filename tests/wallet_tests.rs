use rblc::transaction::Transaction;

#[test]
fn test_transaction_creation() {
    let transaction = Transaction::new("user1".to_string(), "user2".to_string(), 50);
    assert_eq!(transaction.from, "user1");
    assert_eq!(transaction.to, "user2");
    assert_eq!(transaction.amount, 50);
}
