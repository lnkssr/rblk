use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use tokio;

#[derive(Deserialize, Debug)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
}

pub struct WalletClient {
    base_url: String,
    client: Client,
}

impl WalletClient {
    /// Создаем новый клиент для работы с API кошельков.
    pub fn new(base_url: &str) -> Self {
        WalletClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    /// Создаем новый кошелек, отправляя запрос к API.
    pub async fn create_wallet(&self) -> Result<Wallet, Box<dyn Error>> {
        let url = format!("{}/wallet", self.base_url);
        let response = self.client.post(&url).send().await?;

        if response.status().is_success() {
            let wallet = response.json::<Wallet>().await?;
            Ok(wallet)
        } else {
            Err(format!("Failed to create wallet: {}", response.status()).into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Создаем клиента для обращения к API
    let wallet_client = WalletClient::new("http://127.0.0.1:3000");

    // Делаем запрос на создание кошелька
    match wallet_client.create_wallet().await {
        Ok(wallet) => println!("Created new wallet: {:?}", wallet),
        Err(e) => println!("Error creating wallet: {}", e),
    }

    Ok(())
}
