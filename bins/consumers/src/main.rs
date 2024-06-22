use std::error::Error;

use services::{
    amqp::{consumer_rabbitmq, setup_rabbitmq},
    envs::environment_setup,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = environment_setup();
    info!("Environment Setup Realized");
    let channel = setup_rabbitmq().await?;
    info!("RabbitMQ Setup Realized");

    consumer_rabbitmq(channel).await?;
    Ok(())
}
