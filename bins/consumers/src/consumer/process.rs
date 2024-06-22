use std::sync::Arc;

use async_trait::async_trait;
use services::amqp::{traits::ConsumerHandler, AmqpError, AmqpMessage};
use tracing::info;

pub struct ConsumerMessage {}

impl ConsumerMessage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

#[async_trait]
impl ConsumerHandler for ConsumerMessage {
    async fn handle(&self, msg: AmqpMessage) -> Result<(), AmqpError> {
        info!("consuming message");
        Ok(())
    }
}
