use std::env;

use gigachat_rust::{client::GigaChatClientBuilder, embeddings::Embeddings};

#[tokio::main]
async fn main() {
    let token = env::var("GIGACHAT_TOKEN").expect("GIGACHAT_TOKEN environment variable not set");
    let client = GigaChatClientBuilder::new(token).build().await.unwrap();

    let embeddings = Embeddings::new(client);

    // Single string embedding
    let single_embedding = embeddings
        .create("This is a test string for embedding".to_string(), None)
        .await
        .unwrap();

    println!("Single embedding: {:?}", single_embedding);

    // Multiple strings embedding
    let multiple_embeddings = embeddings
        .create(
            vec![
                "First test string".to_string(),
                "Second test string".to_string(),
                "Third test string".to_string(),
            ],
            None,
        )
        .await
        .unwrap();

    println!("Multiple embeddings: {:?}", multiple_embeddings);
}
