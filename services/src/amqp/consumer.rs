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

use super::{dispatcher::AmqpDispatcherDefinition, errors::AmqpError, AmqpMessage};

pub async fn consume(
    delivery: &Delivery,
    defs: &HashMap<String, AmqpDispatcherDefinition>,
    channel: Arc<Channel>,
) -> Result<(), AmqpError> {
    // get message type
    let msg_type = get_header_msg_type(&delivery.properties);

    // verify in hashmap if have some dispatcher to process this message type
    let Some(dispatcher) = defs.get(&msg_type) else {
        info!("Removing message for unsuported type = {}", msg_type);
        match delivery.ack(BasicAckOptions { multiple: false }).await {
            Err(err) => {
                error!("Error to ACK message , {}", err)
            }
            _ => {}
        }
        return Err(AmqpError::UnsupportedMessageType);
    };

    // get message
    let message = AmqpMessage::new(msg_type, &delivery.data, None);

    // call the handler configured to message type
    match dispatcher.handler.handle(message).await {
        Ok(_) => {
            info!("message sucessfull handled and processed");
            // if sucess ack message
            match delivery.ack(BasicAckOptions { multiple: false }).await {
                Err(e) => {
                    error!("Error to Ack message");
                    return Err(AmqpError::AckError);
                }
                _ => return Ok(()),
            }
        }
        Err(_) => {
            error!("Error to process the message");
            // if some error in process message, delivery the error message to a new queue, for debug latter
            // todo to recorrery error.
            return Err(AmqpError::InternalError);
        }
    };
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
