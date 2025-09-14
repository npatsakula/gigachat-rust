use futures::{StreamExt, TryStreamExt};
use gigachat_rust::{client::GigaChatClientBuilder, generation::structures::Message};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GIGACHAT_TOKEN").expect("GIGACHAT_TOKEN environment variable not set");
    let client = GigaChatClientBuilder::new(token).build().await.unwrap();

    let check = client
        .generate()
        .with_messages(vec![
            Message::system("Переведи документацию на английский язык."),
            Message::user(include_str!("../data/short.txt")),
        ])
        .execute()
        .await
        .unwrap();

    println!("{check:?}");

    let check = client
        .generate()
        .with_messages(vec![
            Message::system("Переведи документацию на английский язык."),
            Message::user(include_str!("../data/short.txt")),
        ])
        .execute_streaming()
        .await
        .unwrap();

    check
        .enumerate()
        .map(|(i, r)| r.map(|r| (i, r)))
        .try_for_each(async |(i, response)| {
            println!("Part {i}:\n{response:?}");
            Ok(())
        })
        .await
        .unwrap();
}
