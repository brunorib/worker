use std::env;
extern crate dotenv;

pub struct Config {
    pub rabbitmq_url: String,
    pub rabbitmq_queue: String,
    pub keystore_path: String,
    pub keystore_pass: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        dotenv::dotenv().expect("Failed to read .env file");
        let rabbitmq_url = env::var("RABBIT_MQ_URL").unwrap();
        let rabbitmq_queue = env::var("RABBIT_MQ_QUEUE").unwrap();
        let keystore_path = env::var("KEYSTORE_PATH").unwrap();
        let keystore_pass = env::var("KEYSTORE_PASS").unwrap();
        
        Ok(Config { rabbitmq_url, rabbitmq_queue, keystore_path, keystore_pass })
    }
}