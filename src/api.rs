use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};

pub async fn create_wallet(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    let wallet = Wallet::new();
    blockchain.set_wallet_balance(&wallet.address, 100);
    HttpResponse::Created().json(wallet)
}

pub async fn get_balance(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    address: web::Path<String>,
) -> impl Responder {
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

pub async fn add_block(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    data: web::Json<String>,
    miner_address: web::Json<String>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    blockchain.add_block(data.into_inner(), miner_address.into_inner(), vec![]);
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
    blockchain
        .save_wallets_to_file()
        .expect("Error saving wallets");
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
/*
### Список команд API

1. **Создание кошелька**
   - **Метод**: `POST /wallet`
   - **Описание**: Создаёт новый кошелёк с начальным балансом 100.

2. **Получение баланса кошелька**
   - **Метод**: `GET /balance/{address}`
   - **Описание**: Возвращает баланс кошелька по указанному адресу.

3. **Создание транзакции**
   - **Метод**: `POST /transaction`
   - **Описание**: Создаёт новую транзакцию, проверяя, достаточно ли средств на кошельке отправителя.

4. **Добавление нового блока**
   - **Метод**: `POST /block`
   - **Описание**: Добавляет новый блок в блокчейн с указанными данными и адресом майнера.

5. **Проверка валидности блокчейна**
   - **Метод**: `GET /chain/validity`
   - **Описание**: Проверяет, является ли блокчейн валидным.

6. **Сохранение состояния блокчейна**
   - **Метод**: `POST /chain/save`
   - **Описание**: Сохраняет текущее состояние блокчейна и кошельков в файлы.

7. **Загрузка состояния блокчейна**
   - **Метод**: `POST /chain/load`
   - **Описание**: Загружает состояние блокчейна и кошельков из файлов.

### Примеры использования команд

- **Создание кошелька**:
  ```sh
  curl -X POST http://127.0.0.1:3000/wallet
  ```

- **Получение баланса**:
  ```sh
  curl http://127.0.0.1:3000/balance/{адрес_кошелька}
  ```

- **Создание транзакции**:
  ```sh
  curl -X POST http://127.0.0.1:3000/transaction -H "Content-Type: application/json" -d '{"from": "адрес_отправителя", "to": "адрес_получателя", "amount": 10}'
  ```

- **Добавление блока**:
  ```sh
  curl -X POST http://127.0.0.1:3000/block -H "Content-Type: application/json" -d '{"data": "Данные для блока", "miner_address": "адрес_майнера"}'
  ```

- **Проверка валидности блокчейна**:
  ```sh
  curl http://127.0.0.1:3000/chain/validity
  ```

- **Сохранение состояния**:
  ```sh
  curl -X POST http://127.0.0.1:3000/chain/save
  ```

- **Загрузка состояния**:
  ```sh
  curl -X POST http://127.0.0.1:3000/chain/load
  ```

Этот список охватывает весь функционал вашего API и может служить справочным материалом для работы с ним.
*/
