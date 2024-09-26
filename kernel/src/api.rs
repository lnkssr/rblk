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
    let wallet = Wallet::new("some_address".to_string());
    blockchain.wallets.push(wallet.clone());
    HttpResponse::Created().json(wallet)
}

pub async fn get_balance(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    address: web::Path<String>,
) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    match blockchain.get_wallet(&address) {
        Some(wallet_index) => {
            let wallet = &blockchain.wallets[wallet_index];
            HttpResponse::Ok().json(wallet.get_balance())
        },
        None => HttpResponse::BadRequest().body("Wallet not found"),
    }
}

pub async fn create_transaction(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    transaction: web::Json<Transaction>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    match blockchain.execute_transaction(&transaction) {
        Ok(_) => HttpResponse::Created().json("Transaction created"),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn add_block(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    request: web::Json<AddBlockRequest>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    blockchain.add_block(request.data.clone(), request.miner_address.clone());
    HttpResponse::Created().json("Block added")
}

pub async fn check_chain_validity(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    if blockchain.is_chain_valid() {
        HttpResponse::Ok().body("Blockchain is valid")
    } else {
        HttpResponse::BadRequest().body("Blockchain is invalid")
    }
}

pub async fn save_chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    if blockchain.save_to_files().is_ok() {
        HttpResponse::Ok().body("Blockchain saved")
    } else {
        HttpResponse::InternalServerError().body("Error saving blockchain")
    }
}

pub async fn load_chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    *blockchain = Blockchain::new();
    HttpResponse::Ok().body("Blockchain loaded")
}

pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>) {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .service(
                web::scope("/api")
                    .route("/wallet", web::post().to(create_wallet))
                    .route("/balance/{address}", web::get().to(get_balance))
                    .route("/transaction", web::post().to(create_transaction))
                    .route("/block", web::post().to(add_block))
                    .route("/chain/validity", web::get().to(check_chain_validity))
                    .route("/chain/save", web::post().to(save_chain))
                    .route("/chain/load", web::post().to(load_chain)),
            )
    })
    .bind("127.0.0.1:3000")
    .expect("Failed to bind to address")
    .run()
    .await
    .unwrap();
}
