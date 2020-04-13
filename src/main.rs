extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_big_array;

mod client;
mod key_store;
mod signer;
mod message_controller;
mod worker;
mod config;
mod commons;

use env_logger;
use log::{error, info};
use openssl::pkcs12::ParsedPkcs12;
use config::Config;

fn main() {
    env_logger::init();

    let config: Config = Config::new().unwrap();
    info!("Successfully read env variables");
    let keystore: ParsedPkcs12 = key_store::read_pkcs12(config.keystore_path.clone(), config.keystore_pass.clone());

    let result = worker::listen(config, &keystore);
    match result {
        Ok(_v) => info!("Service stopped"),
        Err(e) => error!("{}", e),
    }
}
