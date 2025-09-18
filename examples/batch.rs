use display_error_chain::DisplayErrorChain;
use gigachat_rust::{client::GigaChatClientBuilder, error::*, generation::structures::Message};
use snafu::ResultExt;
use std::{env, process::ExitCode, time::Duration};
use tokio::time::sleep;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

async fn do_main() -> Result<(), Error> {
    let token = env::var("GIGACHAT_TOKEN").whatever_context("GIGACHAT_TOKEN must be set")?;
    let client = GigaChatClientBuilder::new(token)
        .build()
        .await
        .context(ClientSnafu)?;

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
        .context(BatchSnafu)?;

    tracing::info!(batch.id = handler.id(), "batch job started");

    loop {
        let status = handler.check().await.context(BatchSnafu)?;
        match status {
            gigachat_rust::batch::handler::BatchCheckResult::Pending => {
                tracing::info!("batch is pending...");
            }
            gigachat_rust::batch::handler::BatchCheckResult::InProgress { ready, total } => {
                tracing::info!(ready = ready, total = total, "batch in progress");
            }
            gigachat_rust::batch::handler::BatchCheckResult::Success { responses } => {
                tracing::info!("batch completed successfully");
                for (i, response_result) in responses.into_iter().enumerate() {
                    match response_result.res() {
                        Ok(response) => {
                            tracing::info!(response.index = i, response = ?response, "response received")
                        }
                        Err(e) => {
                            tracing::error!(response.index = i, error = ?e, "response failed")
                        }
                    }
                }
                break;
            }
        }
        sleep(Duration::from_secs(5)).await;
    }

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
