use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{self, Duration},
};

use async_trait::async_trait;
use services::{
    amqp::{traits::ConsumerHandler, AmqpError, AmqpMessage},
    messages::ProcessDataTimer,
};

use tokio::{sync::mpsc, time::sleep};
use tracing::{debug, error, info};

pub struct ConsumerMessage {
    timers: Arc<Mutex<HashMap<i32, String>>>,
}

impl ConsumerMessage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            timers: Arc::new(Mutex::new(HashMap::new())),
        })
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

        {
            let mut timers = self.timers.lock().unwrap();

            match timers.get(&data.id.clone()) {
                Some(val) => {
                    debug!("Exist value process to id = {}", &val);
                    return Ok(());
                }
                None => {
                    debug!("Add to hashmap process to id = {}", &data.id);
                    timers.insert(data.id.clone(), data.message.clone());
                }
            };
        }

        let timers = Arc::clone(&self.timers);

        tokio::spawn(async move {
            process_message(data.clone()).await;
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
