use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub address: String,
    balance: u64,
}

impl Wallet {
    pub fn new(address: String) -> Self {
        Self { address, balance: 0 }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }
}
