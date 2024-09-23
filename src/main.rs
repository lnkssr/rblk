use rblk::api::start_server;
use rblk::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    start_server(blockchain).await;
}
