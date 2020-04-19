use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result, Exchange, Publish, AmqpProperties};
use log::{info, warn};
use serde_json::Value;
use openssl::pkcs12::ParsedPkcs12;

use crate::message_controller;
use crate::config::Config;

fn run_connection(mut connection: Connection, keystore: &ParsedPkcs12, config: &Config) -> Result<()> {
    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(config.rabbitmq_queue.clone(), QueueDeclareOptions::default())?;
    // Get a handle to the default direct exchange.
    let exchange = Exchange::direct(&channel);
    let consumer = queue.consume(ConsumerOptions::default())?;
    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                info!("Received [{}]", body);
                

                let (reply_to, corr_id) = match (
                    delivery.properties.reply_to(),
                    delivery.properties.correlation_id(),
                ) {
                    (Some(r), Some(c)) => (r.clone(), c.clone()),
                    _ => {
                        warn!("received delivery without reply_to or correlation_id");
                        consumer.ack(delivery)?;
                        continue;
                    }
                };

                let message: Value = serde_json::from_str(&body).unwrap();
                let response = message_controller::handle(message, &keystore).unwrap();

                exchange.publish(Publish::with_properties(
                    response.as_bytes(),
                    reply_to,
                    AmqpProperties::default().with_correlation_id(corr_id),
                ))?;
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
    run_connection(connection, &keystore, &config)
}
