use lapin;

#[tokio::main]
async fn main() {
    let connection = lapin::Connection::connect(
        "amqp://127.0.0.1:5672/%2f",
        lapin::ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ");

    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create channel");

    channel
        .queue_declare(
            "hello".into(),
            lapin::options::QueueDeclareOptions::default(),
            Default::default(),
        )
        .await
        .expect("Failed to declare 'hello' queue");

    println!("Publishing message...");

    channel
        .basic_publish(
            "".into(),
            "hello".into(),
            lapin::options::BasicPublishOptions::default(),
            "Hello, World!".as_bytes(),
            lapin::BasicProperties::default(),
        )
        .await
        .expect("Failed to publish")
        .await
        .expect("Failed to confirm publish");
}
