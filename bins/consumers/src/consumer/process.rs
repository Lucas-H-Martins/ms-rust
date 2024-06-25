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

use tokio::{
    sync::{mpsc, oneshot},
    time::sleep,
};
use tracing::{debug, error, info};

pub struct ConsumerMessage {
    timers: Arc<Mutex<HashMap<i32, (String, oneshot::Sender<()>)>>>,
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

        debug!("Received data to process {:?}", data);

        // change to some method to verify if need to cancel rx channel or no based on message id
        {
            let mut timers = self.timers.lock().unwrap();

            if let Some((_, cancel_handle)) = timers.remove(&data.id) {
                let _ = cancel_handle.send(());

                debug!("Cancelled existing timer for id = {}", data.id);
                return Ok(());
            }

            let (cancel_tx, cancel_rx) = oneshot::channel();

            timers.insert(data.id, (data.message.clone(), cancel_tx));

            let timers = Arc::clone(&self.timers);

            tokio::spawn(async move {
                process_message(data, timers, cancel_rx).await;
            });
        }

        Ok(())
    }
}
async fn process_message(
    data: ProcessDataTimer,
    timers: Arc<Mutex<HashMap<i32, (String, oneshot::Sender<()>)>>>,
    mut cancel_rx: oneshot::Receiver<()>,
) {
    info!("Starting TTL await for id = {}", data.id);

    tokio::select! {
        _ = sleep(Duration::from_secs(data.ttl as u64)) => {
            info!("TTL completed for id = {}", data.id);
            let mut timers = timers.lock().unwrap();
            if let Some((message, _)) = timers.remove(&data.id) {
                info!("Processing completed for id = {}, message = {}", data.id, message);
            }
        }
        _ = &mut cancel_rx => {
            info!("TTL cancelled for id = {}", data.id);
        }
    }
}
