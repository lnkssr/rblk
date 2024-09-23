use actix_web::{web, HttpResponse, Responder, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub async fn create_wallet(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap(); 
    let wallet = Wallet::new();
    blockchain.set_wallet_balance(&wallet.address, 100); 
    HttpResponse::Created().json(wallet)
}

pub async fn get_balance(blockchain: web::Data<Arc<Mutex<Blockchain>>>, address: web::Path<String>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    let balance = blockchain.get_wallet_balance(&address);
    HttpResponse::Ok().json(balance)
}

pub async fn create_transaction(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    transaction: web::Json<Transaction>,
) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    if blockchain.get_wallet_balance(&transaction.from) >= transaction.amount {
        HttpResponse::Created().json("Transaction created")
    } else {
        HttpResponse::BadRequest().json("Insufficient funds")
    }
}

pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>) {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .route("/wallet", web::post().to(create_wallet))
            .route("/balance/{address}", web::get().to(get_balance))
            .route("/transaction", web::post().to(create_transaction))
    })
    .bind("127.0.0.1:3000")
    .expect("Failed to bind to address")
    .run()
    .await
    .unwrap();
}
