use gigachat_rust::{client::GigaChatClientBuilder, generation::structures::Message};
use std::{env, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let token = env::var("GIGACHAT_TOKEN").expect("GIGACHAT_TOKEN environment variable not set");
    let client = GigaChatClientBuilder::new(token).build().await.unwrap();

    let handler = client
        .batch()
        .with_request(
            client
                .generate()
                .with_messages(vec![Message::user("What is the capital of France?")])
                .build(),
        )
        .with_request(
            client
                .generate()
                .with_messages(vec![Message::user("What is the largest planet?")])
                .build(),
        )
        .execute()
        .await
        .unwrap();

    println!("Batch job started with ID: {}", handler.id());

    loop {
        let status = handler.check().await.unwrap();
        match status {
            gigachat_rust::batch::handler::BatchCheckResult::Pending => {
                println!("Batch is pending...");
            }
            gigachat_rust::batch::handler::BatchCheckResult::InProgress { ready, total } => {
                println!("Batch in progress: {}/{} ready", ready, total);
            }
            gigachat_rust::batch::handler::BatchCheckResult::Success { responses } => {
                println!("Batch completed successfully!");
                for (i, response_result) in responses.into_iter().enumerate() {
                    match response_result.res() {
                        Ok(response) => println!("Response {}: {:?}", i, response),
                        Err(e) => println!("Response {} failed with error: {:?}", i, e),
                    }
                }
                break;
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}
