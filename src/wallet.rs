use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use rand::Rng;
use hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            address: Self::generate_private_key(),
            balance: 0,
        }
    }
    pub fn generate_address(&self) -> &str {
        self.address.as_ref()
    }

    pub fn generate_public_key(&self, private_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(private_key);
        let public_key = hasher.finalize();
        hex::encode(public_key)
    }

    pub fn generate_private_key() -> String {
        let mut rng = rand::thread_rng();
        let private_key: [u8; 32] = rng.gen(); 
        hex::encode(private_key)
    }
}