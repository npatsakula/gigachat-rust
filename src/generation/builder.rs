use eventsource_stream::Eventsource;
use futures::{Stream, StreamExt, TryStreamExt};
use snafu::prelude::*;
use std::future;
use tracing::Span;

use super::{
    error,
    structures::{GenerationRequest, GenerationResponse, GenerationResponseStream, Message},
};
use crate::{
    client::GigaChatClient,
    function::{FunctionName, UserFunction},
    generation::structures::{Function, FunctionCall},
};

pub struct GenerationBuilder {
    client: GigaChatClient,

    model: super::Model,
    messages: Option<Vec<super::structures::Message>>,
    config: super::structures::GenerationConfig,
    functions: Vec<Function>,
    function_call: FunctionCall,
}

impl GenerationBuilder {
    pub fn with_model(mut self, model: super::Model) -> Self {
        self.model = model;
        self
    }

    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.config.temperature = Some(temperature);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.config.top_p = Some(top_p);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.config.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_repetition_penalty(mut self, repetition_penalty: f32) -> Self {
        self.config.repetition_penalty = Some(repetition_penalty);
        self
    }

    pub fn with_user_function(mut self, user_function: UserFunction) -> Self {
        self.functions.push(Function::User(user_function));
        self
    }

    pub fn with_builtin_function(mut self, function: FunctionName) -> Self {
        self.functions.push(Function::BuiltIn(function));
        self
    }

    pub fn with_function_call(mut self, function_call: FunctionCall) -> Self {
        self.function_call = function_call;
        self
    }

    pub fn build(self) -> GenerationRequest {
        GenerationRequest {
            model: self.model,
            messages: self.messages.unwrap_or_default(),
            config: self.config,
            function_call: self.function_call,
            functions: self.functions,
        }
    }

    #[tracing::instrument(skip_all, fields(url))]
    pub async fn execute(self) -> Result<GenerationResponse, error::Error> {
        let client = self.client.clone();
        let request = self.build();

        let url = client
            .build_url("chat/completions", None)
            .context(error::BuildUrlSnafu)?;
        Span::current().record("url", url.as_str());
        tracing::debug!("URL constructed successfully");

        client
            // .perform_request(|c| c.post(url).json(&request), async |r| r.json().await)
            .perform_request(
                |c| c.post(url).json(&request),
                async |r| {
                    let response = r.json::<serde_json::Value>().await?;
                    println!("{response}");

                    Ok(serde_json::from_value(response).unwrap())
                },
            )
            .await
            .context(error::BadRequestSnafu)
    }

    #[tracing::instrument(skip_all, fields(url))]
    pub async fn execute_streaming(
        mut self,
    ) -> Result<impl Stream<Item = Result<GenerationResponseStream, error::Error>>, error::Error>
    {
        let client = self.client.clone();
        self.config.stream = true;
        let request = self.build();

        let url = client
            .build_url("chat/completions", None)
            .context(error::BuildUrlSnafu)?;
        Span::current().record("url", url.as_str());
        tracing::debug!("URL constructed successfully");

        let stream = client
            .perform_request(|c| c.post(url).json(&request), async |r| Ok(r))
            .await
            .context(error::BadRequestSnafu)?;

        Ok(stream
            .bytes_stream()
            .eventsource()
            .take_while(|event| {
                // FIXME:
                // This is obvious SSE misuse from the Sber team, this code shouldn't
                // exists, but here we are.
                future::ready(matches!(event, Ok(event) if event.data != "[DONE]"))
            })
            .map(|r| r.context(error::EventParseFailedSnafu))
            .map_ok(|event| {
                serde_json::from_str(&event.data).context(error::StreamDeserializationFailedSnafu {
                    event_data: event.data,
                })
            })
            .map(|r| r.flatten()))
    }
}

impl GigaChatClient {
    pub fn generate(&self) -> GenerationBuilder {
        GenerationBuilder {
            client: self.clone(),
            model: super::Model::default(),
            messages: None,
            config: Default::default(),
            function_call: FunctionCall::default(),
            functions: Vec::new(),
        }
    }
}
