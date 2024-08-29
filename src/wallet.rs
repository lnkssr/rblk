use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new(address: String) -> Self {
        Wallet {
            address,
            balance: 0,
        }
    }
}
