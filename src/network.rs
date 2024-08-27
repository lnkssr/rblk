use std::thread;
use std::time::Duration;

pub struct Network;

impl Network {
    pub fn new() -> Self {
        Network
    }

    pub fn start(&self) {
        // Простейший пример работы сетевого слоя
        thread::spawn(move || {
            loop {
                // Здесь будет код для обработки входящих и исходящих сообщений
                println!("Network is running...");
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
}
