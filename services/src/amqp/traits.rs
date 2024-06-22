use std::sync::Arc;

use async_trait::async_trait;

use super::{dispatcher::DispatcherDefinition, errors::AmqpError, handler::AmqpMessage};

#[async_trait]
pub trait ConsumerHandler: Send + Sync {
    async fn handle(&self, msg: AmqpMessage) -> Result<(), AmqpError>;
}

#[async_trait]
pub trait Dispatcher: Send + Sync {
    async fn register(
        self,
        definition: &DispatcherDefinition,
        handler: Arc<dyn ConsumerHandler>,
    ) -> Self;

    async fn consumer_data(self) -> Result<(), AmqpError>;
}
