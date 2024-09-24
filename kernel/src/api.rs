use crate::transaction::Transaction;
use crate::{blockchain::Blockchain, wallet::Wallet};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct AddBlockRequest {
    pub data: String,
    pub miner_address: String,
}

pub async fn create_wallet(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    let wallet = Wallet::new();
    blockchain.get_wallet(&wallet.address);
    HttpResponse::Created().json(wallet)
}

pub async fn get_balance(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    address: web::Path<String>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    let wallet_index = blockchain.get_wallet(&address);
    let balance = blockchain.wallets[wallet_index].get_balance();
    HttpResponse::Ok().json(balance)
}

pub async fn create_transaction(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    transaction: web::Json<Transaction>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    let sender_index = blockchain.get_wallet(&transaction.from);
    let receiver_index = blockchain.get_wallet(&transaction.to);

    let (sender_wallet, receiver_wallet) = if sender_index < receiver_index {
        let (left, right) = blockchain.wallets.split_at_mut(receiver_index);
        (&mut left[sender_index], &mut right[0])
    } else {
        let (left, right) = blockchain.wallets.split_at_mut(sender_index);
        (&mut right[0], &mut left[receiver_index])
    };

    if sender_wallet.get_balance() >= transaction.amount {
        sender_wallet.set_balance(sender_wallet.get_balance() - transaction.amount);
        receiver_wallet.set_balance(receiver_wallet.get_balance() + transaction.amount);
        HttpResponse::Created().json("Transaction created")
    } else {
        HttpResponse::BadRequest().json("Insufficient funds")
    }
}

pub async fn add_block(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    request: web::Json<AddBlockRequest>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    blockchain.add_block(request.data.clone(), request.miner_address.clone(), vec![]);
    HttpResponse::Created().json("Block added")
}

pub async fn check_chain_validity(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    let is_valid = blockchain.is_chain_valid();
    if is_valid {
        HttpResponse::Ok().body("Blockchain is valid")
    } else {
        HttpResponse::BadRequest().body("Blockchain is invalid")
    }
}

pub async fn save_chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    blockchain.save_to_files().expect("Error saving blockchain");
    blockchain.save_to_files().expect("Error saving wallets");
    HttpResponse::Ok().body("Blockchain saved")
}

pub async fn load_chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    *blockchain = Blockchain::load_from_files().unwrap_or_else(|_| Blockchain::new());
    HttpResponse::Ok().body("Blockchain loaded")
}

pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>) {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .route("/wallet", web::post().to(create_wallet))
            .route("/balance/{address}", web::get().to(get_balance))
            .route("/transaction", web::post().to(create_transaction))
            .route("/block", web::post().to(add_block))
            .route("/chain/validity", web::get().to(check_chain_validity))
            .route("/chain/save", web::post().to(save_chain))
            .route("/chain/load", web::post().to(load_chain))
    })
    .bind("127.0.0.1:3000")
    .expect("Failed to bind to address")
    .run()
    .await
    .unwrap();
}
