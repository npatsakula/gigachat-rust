use display_error_chain::DisplayErrorChain;
use futures::{StreamExt, TryStreamExt};
use gigachat_rust::{
    client::GigaChatClientBuilder,
    error::*,
    generation::{Model, structures::Message},
};
use snafu::prelude::*;
use std::{env, process::ExitCode};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

async fn do_main() -> Result<(), Error> {
    let token = env::var("GIGACHAT_TOKEN").whatever_context("GIGACHAT_TOKEN must be set")?;

    let client = GigaChatClientBuilder::new(token)
        .build()
        .await
        .context(ClientSnafu)?;

    let generated = client
        .generate()
        .with_model(Model::GigaChat2Lite)
        .with_messages(vec![
            Message::system("Переведи документацию на английский язык."),
            Message::user(include_str!("../data/short.txt")),
        ])
        .execute()
        .await
        .context(GenerateSnafu)?;

    tracing::info!(generated = ?generated, "response generated successfully");

    let check = client
        .generate()
        .with_model(Model::GigaChat2Lite)
        .with_messages(vec![
            Message::system("Переведи документацию на английский язык."),
            Message::user(include_str!("../data/short.txt")),
        ])
        .execute_streaming()
        .await
        .context(GenerateSnafu)?;

    check
        .enumerate()
        .map(|(i, r)| r.map(|r| (i, r)))
        .try_for_each(async |(i, response)| {
            tracing::info!(part.i = i, part.response = ?response, "part generated successfully");
            Ok(())
        })
        .await
        .context(GenerateSnafu)?;

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
        tracing::error!(error.debug = ?err, error.chain = error_chain, "top level error");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
