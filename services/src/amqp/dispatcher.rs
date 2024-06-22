use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use futures_lite::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Channel};
use tracing::{error, info};

use super::{
    consume,
    errors::AmqpError,
    traits::{ConsumerHandler, Dispatcher},
    QueueDefinition,
};

pub struct DispatcherDefinition {
    pub name: String,
    pub msg_type: String,
}

impl DispatcherDefinition {
    pub fn new<T>(name: T, msg_type: T) -> Self
    where
        T: Into<String>,
    {
        DispatcherDefinition {
            name: name.into(),
            msg_type: msg_type.into(),
        }
    }
}

pub struct AmqpDispatcherDefinition {
    pub queue_def: QueueDefinition,
    pub handler: Arc<dyn ConsumerHandler>,
}

pub struct AmqpDispatcher {
    channel: Arc<Channel>,
    queue_def: Vec<QueueDefinition>,
    pub dispatcher_def: HashMap<String, AmqpDispatcherDefinition>,
}

impl AmqpDispatcher {
    pub fn new(channel: Arc<Channel>, queue_def: Vec<QueueDefinition>) -> Self {
        AmqpDispatcher {
            channel,
            queue_def,
            dispatcher_def: HashMap::default(),
        }
    }
}

#[async_trait]
impl Dispatcher for AmqpDispatcher {
    async fn register(
        mut self,
        def: &DispatcherDefinition,
        handler: Arc<dyn ConsumerHandler>,
    ) -> Self {
        let mut queue_def = QueueDefinition::default();

        for queue in &self.queue_def {
            if def.name == queue.name.to_string() {
                queue_def = queue.clone();
            }
        }

        self.dispatcher_def.insert(
            def.msg_type.clone(),
            AmqpDispatcherDefinition { queue_def, handler },
        );

        self
    }

    async fn consumer_data(self) -> Result<(), AmqpError> {
        self.consume_single_handler().await
    }
}

impl AmqpDispatcher {
    pub async fn consume_single_handler(self) -> Result<(), AmqpError> {
        let key = self.dispatcher_def.keys().next().unwrap();
        let def = self.dispatcher_def.get(key).unwrap();

        let mut consumer = match self
            .channel
            .basic_consume(
                &def.queue_def.name.to_string(),
                &key,
                BasicConsumeOptions {
                    nowait: false,
                    exclusive: false,
                    no_ack: false,
                    no_local: false,
                },
                FieldTable::default(),
            )
            .await
        {
            Ok(c) => Ok(c),
            Err(err) => {
                error!("create consumer error: {}", err);
                Err(AmqpError::ConsumerError)
            }
        }?;

        let defs = self.dispatcher_def;

        let channel = self.channel.clone();

        let spawned = tokio::spawn({
            async move {
                while let Some(result) = consumer.next().await {
                    match result {
                        Ok(delivery) => {
                            //consume data
                            match consume(&delivery, &defs, channel.clone()).await {
                                Err(err) => {
                                    error!("Error consumer message: {}", err)
                                }
                                _ => {}
                            }
                        }
                        Err(_) => todo!(),
                    }
                }
            }
        })
        .await;
        if (spawned.is_err()) {
            error!("Internal Error");
            return Err(AmqpError::InternalError);
        }

        return Ok(());
    }
}
