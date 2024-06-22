use std::{env, error::Error};

use futures_lite::stream::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{error, info};

pub async fn setup_rabbitmq() -> Result<Channel, Box<dyn Error>> {
    let amqp_host = env::var("AMQP_HOST").unwrap_or_else(|_| "localhost".to_string());
    let amqp_port = env::var("AMQP_PORT").unwrap_or_else(|_| "5671".to_string());
    let amqp_user = env::var("AMQP_USER").unwrap_or_else(|_| "guest".to_string());
    let amqp_password = env::var("AMQP_PASSWORD").unwrap_or_else(|_| "guest".to_string());

    let amqp_uri = format!(
        "amqp://{}:{}@{}:{}",
        amqp_user, amqp_password, amqp_host, amqp_port
    );

    info!("Initialize RabbitMQ Connection {}", { &amqp_uri.clone() });

    let conn = Connection::connect(&amqp_uri.clone(), ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;

    channel
        .exchange_declare(
            "exchange_name",
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let queue_name = "my_queue";
    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            queue_name,
            "exchange_name",
            "routing_key",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;
    Ok(channel)
}

pub async fn consumer_rabbitmq(channel: Channel) -> Result<(), Box<dyn Error>> {
    // Consuma mensagens da fila
    let mut consumer = channel
        .basic_consume(
            "my_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");

        info!("Received data");
        match std::str::from_utf8(&delivery.data) {
            Ok(message) => {
                //
                info!("Received message: {}", message);
            }
            Err(_) => {
                //
                error!("Received non-UTF8 message");
            }
        }

        delivery.ack(BasicAckOptions::default()).await?
    }
    println!("Consumer created");

    Ok(())
}
