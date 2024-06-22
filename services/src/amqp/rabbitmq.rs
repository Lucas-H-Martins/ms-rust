use std::{env, error::Error, sync::Arc};

use lapin::{
    options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{error, info};

pub async fn amqp_channel() -> Result<Arc<Channel>, Box<dyn Error>> {
    let amqp_host = env::var("AMQP_HOST").unwrap_or_else(|_| "localhost".to_string());
    let amqp_port = env::var("AMQP_PORT").unwrap_or_else(|_| "5671".to_string());
    let amqp_user = env::var("AMQP_USER").unwrap_or_else(|_| "guest".to_string());
    let amqp_password = env::var("AMQP_PASSWORD").unwrap_or_else(|_| "guest".to_string());

    let amqp_uri = format!(
        "amqp://{}:{}@{}:{}",
        amqp_user, amqp_password, amqp_host, amqp_port
    );

    info!("Initialize RabbitMQ Connection {}", { &amqp_uri.clone() });

    let conn = Connection::connect(&amqp_uri.clone(), ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;

    Ok(Arc::new(channel))
}
