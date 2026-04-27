use lapin::{Connection, ConnectionProperties, Result, options::*, types::FieldTable};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;

    channel
        .queue_declare(
            "hello".into(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "hello".into(),
            "my_consumer".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let msg = std::str::from_utf8(&delivery.data).expect("invalid UTF-8");
        println!("Received message: {:?}", msg);
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
