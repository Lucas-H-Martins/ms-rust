use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use services::{
    amqp::{traits::ConsumerHandler, AmqpError, AmqpMessage},
    messages::ProcessDataTimer,
};

use tokio::time::sleep;
use tracing::{error, info};

pub struct ConsumerMessage {}

impl ConsumerMessage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

#[async_trait]
impl ConsumerHandler for ConsumerMessage {
    async fn handle(&self, msg: AmqpMessage) -> Result<(), AmqpError> {
        let data = match serde_json::from_slice::<ProcessDataTimer>(&msg.data) {
            Ok(data) => data,

            Err(err) => {
                error!("Error to retrieve information of message {}", err);
                return Err(AmqpError::ProcessMessage);
            }
        };
        info!("Received data to process {}", data);

        tokio::spawn(async move {
            process_message(data).await;
        });
        Ok(())
    }
}
async fn process_message(data: ProcessDataTimer) {
    info!("Init TTL Await");
    // wait to execute ttl
    sleep(Duration::from_secs(data.ttl as u64)).await;

    info!("End TTL await");
}
