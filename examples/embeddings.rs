use display_error_chain::DisplayErrorChain;
use gigachat_rust::{client::GigaChatClientBuilder, embeddings::Embeddings, error::*};
use snafu::ResultExt;
use std::{env, process::ExitCode};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

async fn do_main() -> Result<(), Error> {
    let token = env::var("GIGACHAT_TOKEN").whatever_context("GIGACHAT_TOKEN must be set")?;
    let client = GigaChatClientBuilder::new(token)
        .build()
        .await
        .context(ClientSnafu)?;

    let embeddings = Embeddings::new(client);

    // Single string embedding
    let single_embedding = embeddings
        .create("This is a test string for embedding".to_string(), None)
        .await
        .context(EmbeddingsSnafu)?;

    tracing::info!(embedding = ?single_embedding, "single embedding created successfully");

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
        .context(EmbeddingsSnafu)?;

    tracing::info!(embeddings = ?multiple_embeddings, "multiple embeddings created successfully");

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    if let Err(err) = do_main().await {
        let error_chain = DisplayErrorChain::new(&err).to_string();
        tracing::error!(error.chain = error_chain, "top level error");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
