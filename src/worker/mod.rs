use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use log::info;
use serde_json::Value;
use openssl::pkcs12::ParsedPkcs12;

use crate::message_controller;
use crate::config::Config;

fn run_connection(mut connection: Connection, keystore: &ParsedPkcs12) -> Result<()> {
    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;
    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                info!("Received [{}]", body);
                let message: Value = serde_json::from_str(&body).unwrap();
                let _response = message_controller::handle(message, &keystore);
                //send that response
                consumer.ack(delivery)?;
            }
            _other => {
                break;
            }
        };
    }

    connection.close()
}

pub fn listen(config: Config, keystore: &ParsedPkcs12) -> Result<()> {
    info!("Starting new worker");
    // Open connection.
    // "amqp://guest:guest@localhost:5672"
    let connection = Connection::insecure_open(&config.rabbitmq_url)?;
    run_connection(connection, &keystore)
}
