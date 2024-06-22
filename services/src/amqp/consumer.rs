use std::{collections::HashMap, sync::Arc};

use futures_lite::StreamExt;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    protocol::basic::AMQPProperties,
    types::{AMQPValue, FieldTable},
    Channel,
};

use tracing::{debug, error, info};

use super::{dispatcher::AmqpDispatcherDefinition, errors::AmqpError};

pub async fn consume(
    delivery: &Delivery,
    defs: &HashMap<String, AmqpDispatcherDefinition>,
    channel: Arc<Channel>,
) -> Result<(), AmqpError> {
    let msg_type = get_header_msg_type(&delivery.properties);

    debug!("Received message type: {:?}", msg_type);

    // Consuma mensagens da fila
    let mut consumer = match channel
        .basic_consume(
            "my_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
    {
        Ok(consumer) => Ok(consumer),
        Err(_) => Err(AmqpError::ConsumerError),
    }?;

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

        match delivery.ack(BasicAckOptions::default()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AmqpError::ConsumerError),
        }?
    }
    Ok(())
}

fn get_header_msg_type(received: &AMQPProperties) -> String {
    let header = received.headers();

    match header {
        Some(header) => {
            for (key, value) in header.inner() {
                if key.to_string() == "type".to_string() {
                    match value {
                        AMQPValue::LongString(s) => return s.to_string().clone(),
                        AMQPValue::ShortString(s) => return s.to_string().clone(),
                        // Handle other possible types if needed
                        _ => continue,
                    }
                }
            }
        }
        None => return "".to_string(),
    }
    "test".to_string()
}
