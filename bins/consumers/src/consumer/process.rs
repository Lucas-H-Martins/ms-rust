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
        match serde_json::from_slice::<ProcessDataTimer>(&msg.data) {
            Ok(data) => {
                info!("Received data to process {}", data);
                // todo create a implementation to call some code here
                execute_task(data);
                return Ok(());
            }
            Err(err) => {
                error!("Error to retrieve information of message {}", err);
                return Err(AmqpError::ProcessMessage);
            }
        };
    }
}
async fn execute_task(data: ProcessDataTimer) {
    println!("Task started: {:?}", data);
    // Simula o tempo de TTL
    sleep(Duration::from_secs(data.ttl as u64)).await;
    // Executa a tarefa após o TTL
    println!("Executing task: {:?}", data);
    // Aqui você pode colocar a lógica da tarefa que deseja executar
}
