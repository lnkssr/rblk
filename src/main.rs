use rblk::api::start_server;
use rblk::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    start_server(blockchain).await;
}
// to do
// 1. get miner commissions
// 3. get network prototype
// 4. add transactions ui sum
