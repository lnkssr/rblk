use std::thread;
use std::time::Duration;

pub struct Network;

impl Network {
    pub fn new() -> Self {
        Network
    }

    pub fn start(&self) {
        thread::spawn(move || {
            loop {
                println!("Network is running...");
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
}
