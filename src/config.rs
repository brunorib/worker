use std::env;

pub struct Config {
    pub rabbitmq_url: String,
    pub keystore_path: String,
    pub keystore_pass: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let rabbitmq_url = env::var("RABBIT_MQ_URL").unwrap();
        let keystore_path = env::var("KEYSTORE_PATH").unwrap();
        let keystore_pass = env::var("KEYSTORE_PASS").unwrap();

        Ok(Config { rabbitmq_url, keystore_path, keystore_pass })
    }
}