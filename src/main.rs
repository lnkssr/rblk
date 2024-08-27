use std::time::{SystemTime, UNIX_EPOCH};

/// Структура для блока
#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

/// Реализация методов для блока
impl Block {
    // Новый блок создается на основе предыдущего блока и данных
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let hash = Block::calculate_hash(index, &data, timestamp, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    // Вычисление хеша для блока
    fn calculate_hash(index: u64, data: &str, timestamp: u128, previous_hash: &str) -> String {
        let block_content = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        format!("{:x}", md5::compute(block_content)) // Используем MD5 для простоты
    }
}

/// Структура для блокчейна
struct Blockchain {
    chain: Vec<Block>,
}

/// Реализация методов для блокчейна
impl Blockchain {
    // Создаем новый блокчейн с генезисным блоком
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    // Получаем последний блок в цепочке
    fn get_latest_block(&self) -> &Block {
        self.chain
            .last()
            .expect("Blockchain should have at least one block")
    }

    // Добавляем новый блок в блокчейн
    fn add_block(&mut self, data: String) {
        let previous_block = self.get_latest_block();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Первый блок после генезиса".to_string());
    blockchain.add_block("Второй блок после генезиса".to_string());

    // Выводим информацию о блоках, включая все поля
    for block in blockchain.chain.iter() {
        println!(
            "Block {{ index: {}, timestamp: {}, data: {}, previous_hash: {}, hash: {} }}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash
        );
    }
}
