mod consumer;
use std::{error::Error, fmt::Display};

use consumer::ConsumerMessage;
use lapin::{options::ExchangeDeclareOptions, protocol::channel, types::FieldTable, ExchangeKind};
use services::{
    amqp::{
        amqp_channel,
        dispatcher::{self, AmqpDispatcher, DispatcherDefinition},
        traits::Dispatcher,
        QueueDefinition,
    },
    envs::environment_setup,
    messages::ProcessDataTimer,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = environment_setup();
    info!("Environment Setup Realized");

    info!("RabbitMQ Setup Realized");

    let dispatcher = amqp_setup().await?;

    match tokio::join!(dispatcher.consumer_data()) {
        (Ok(_),) => todo!(),
        (Err(_),) => todo!(),
        _ => Ok(()),
    }
}

async fn amqp_setup() -> Result<AmqpDispatcher, Box<dyn Error>> {
    let channel = amqp_channel().await?;

    let queue_def = QueueDefinition::new("queue_test").durable();

    let handler = ConsumerMessage::new();

    let dispatcher_def =
        &DispatcherDefinition::new("queue_test", &format!("{}", "ProcessDataTimer"));

    let dispatcher = AmqpDispatcher::new(channel.clone(), vec![queue_def])
        .register(dispatcher_def, handler)
        .await;

    // create exchange
    channel
        .exchange_declare(
            "exchange_test",
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..ExchangeDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;
    channel
        .queue_declare(
            "queue_test",
            lapin::options::QueueDeclareOptions {
                durable: true,
                ..lapin::options::QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;
    channel
        .queue_bind(
            "queue_test",
            "exchange_test",
            "",
            lapin::options::QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;
    Ok(dispatcher)
}
