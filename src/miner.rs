use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub struct Miner {
    pub address: String,
}

impl Miner {
    pub fn new(address: String) -> Miner {
        Miner { address }
    }

    pub fn mine_block(
        &self,
        blockchain: &mut Blockchain,
        data: String,
        transactions: Vec<Transaction>,
    ) {
        let latest_block = blockchain.get_latest_block();
        let mut new_block = Block::new(
            latest_block.index + 1,
            data,
            latest_block.hash.clone(),
            transactions.clone(),
        );

        new_block.mine_block();
        blockchain.add_block(new_block.data.clone(), self.address.clone(), transactions);
    }
}
