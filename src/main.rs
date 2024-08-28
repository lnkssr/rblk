mod block;
mod blockchain;
mod miner;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use miner::Miner;
use std::process;
use transaction::Transaction;

fn main() {
    // Load blockchain from files
    let mut blockchain = Blockchain::load_from_files().unwrap_or_else(|_| Blockchain::new());

    if blockchain.is_chain_valid() {
        println!("bchn is valid");
    } else {
        println!("bchn invalid");
        process::exit(1);
    }

    // Create a miner
    let miner = Miner::new("miner1".to_string());

    // Example transactions
    let transactions = vec![
        Transaction::new("user1".to_string(), "user2".to_string(), 50),
        Transaction::new("user2".to_string(), "user3".to_string(), 25),
    ];

    // Mine new blocks
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

    // Save blockchain and wallets to files
    blockchain
        .save_to_files()
        .expect("Error saving blockchain data");
    blockchain
        .save_wallets_to_file()
        .expect("Error saving wallet data");

    // Print blocks
    for block in blockchain.chain.iter() {
        println!(
            "Block {{ index: {}, timestamp: {}, data: '{}', previous_hash: '{}', hash: '{}', nonce: {}, transactions: {:?} }}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash, block.nonce, block.transactions
        );
    }

    // Print miner balance
    println!(
        "Miner wallet balance {}: {} tokens",
        miner.address,
        blockchain.get_wallet_balance(&miner.address)
    );
}
