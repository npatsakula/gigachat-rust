use display_error_chain::DisplayErrorChain;
use gigachat_rust::{
    client::GigaChatClientBuilder,
    error::*,
    function::{FunctionBuilder, FunctionExample, FunctionExt},
    generation::{
        Model,
        structures::{FunctionCall, Message},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt};
use std::{env, process::ExitCode};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Clone, JsonSchema, Deserialize, Serialize)]
pub struct Route {
    /// Начальное местоположение.
    start_location: String,
    /// Конечное местоположение.
    end_location: String,
}

#[derive(JsonSchema, Deserialize, Serialize)]
pub struct Distance {
    /// Расстояние в километрах.
    kilometers: u32,
}

type DistanceFunction = FunctionBuilder<Route, Distance>;

async fn do_main() -> Result<(), Error> {
    let token = env::var("GIGACHAT_TOKEN").whatever_context("GIGACHAT_TOKEN must be set")?;
    let client = GigaChatClientBuilder::new(token)
        .build()
        .await
        .context(ClientSnafu)?;

    let function = DistanceFunction::new("расстояние")
        .with_description("Расстояние между начальным и конечным местоположением в километрах.")
        .with_example(FunctionExample {
            request: "Насколько далеко от Москвы до Санкт-Петербурга?".to_string(),
            params: Route {
                start_location: "Москва".to_string(),
                end_location: "Санкт-Петербург".to_string(),
            },
        })
        .build();
    let name = function.function_name();

    let warnings = client
        .check_function(&function)
        .await
        .context(FunctionSnafu)?;

    snafu::ensure_whatever!(warnings.is_empty(), "No warnings expected.");

    let mut messages = vec![Message::user(
        "Расстояние между Москвой и Казанью в километрах.",
    )];

    let response = client
        .generate()
        .with_model(Model::GigaChat2Lite)
        .with_messages(messages.clone())
        .with_user_function(function)
        .with_function_call(FunctionCall::Auto)
        .execute()
        .await
        .context(GenerateSnafu)?;

    let response_message = &response
        .choices
        .first()
        .whatever_context("Response must be present.")?
        .message;

    let function_call = match response_message {
        Message::Assistant {
            function_call: Some(fc),
            ..
        } => fc,
        _ => snafu::whatever!("Function call must be present."),
    };

    let deserialized_params = serde_json::from_value::<
        <FunctionBuilder<Route, Distance> as FunctionExt>::Arguments,
    >(function_call.arguments.clone())
    .whatever_context("Failed to deserialize function output")?;

    snafu::ensure_whatever!(
        deserialized_params.start_location == "Москва",
        "Expected start location to be 'Москва' but got: {}",
        deserialized_params.start_location
    );

    snafu::ensure_whatever!(
        deserialized_params.end_location == "Казань",
        "Expected end location to be 'Казань' but got: {}",
        deserialized_params.end_location
    );

    snafu::ensure_whatever!(
        name == function_call.name,
        "Expected function name to be 'route' but got: {:?}",
        function_call.name
    );

    let deserialized_params = serde_json::from_value::<
        <FunctionBuilder<Route, Distance> as FunctionExt>::Arguments,
    >(function_call.arguments.clone())
    .whatever_context("Failed to deserialize function output")?;

    snafu::ensure_whatever!(
        deserialized_params.start_location == "Москва",
        "Expected start location to be 'Москва' but got: {}",
        deserialized_params.start_location
    );

    snafu::ensure_whatever!(
        deserialized_params.end_location == "Казань",
        "Expected end location to be 'Казань' but got: {}",
        deserialized_params.end_location
    );

    messages.push(response_message.clone());
    messages.push(Message::function(name, Distance { kilometers: 842 }));

    let response = client
        .generate()
        .with_messages(messages)
        .with_model(Model::GigaChat2Lite)
        .execute()
        .await
        .context(GenerateSnafu)?;

    snafu::ensure_whatever!(
        response.text().contains("842"),
        "Expected response to contain '842' but got: {}",
        response.text()
    );

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
        tracing::error!(err = ?err, "top level error");
        tracing::error!(error.chain = error_chain, "top level error chain");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
