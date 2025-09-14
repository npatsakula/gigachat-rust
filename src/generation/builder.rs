use anyhow::Context;
use eventsource_stream::Eventsource;
use futures::{Stream, StreamExt, TryStreamExt};
use reqwest::header::ACCEPT_ENCODING;
use serde_json::Value;

use crate::{
    client::GigaChatClient,
    generation::structures::{
        GenerationRequest, GenerationResponse, GenerationResponseStream, Message,
    },
};

pub struct GenerationBuilder {
    client: GigaChatClient,

    model: super::Model,
    messages: Option<Vec<super::structures::Message>>,
    config: super::structures::GenerationConfig,
}

impl GenerationBuilder {
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

    pub async fn execute(self) -> anyhow::Result<GenerationResponse> {
        let request = GenerationRequest {
            model: self.model,
            messages: self.messages.context("trying to execute empty query")?,
            config: self.config,
        };

        let url = self.client.inner.base_url.join("chat/completions")?;
        let result = self
            .client
            .inner
            .client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(serde_json::from_value(result)?)
    }

    pub async fn execute_streaming(
        mut self,
    ) -> anyhow::Result<impl Stream<Item = anyhow::Result<GenerationResponseStream>>> {
        self.config.stream = true;
        let request = GenerationRequest {
            model: self.model,
            messages: self.messages.context("trying to execute empty query")?,
            config: self.config,
        };

        let url = self.client.inner.base_url.join("chat/completions")?;
        let response = self
            .client
            .inner
            .client
            .post(url)
            .header(ACCEPT_ENCODING, "text/event-stream")
            .json(&request)
            .send()
            .await?;

        let response = GigaChatClient::check_response(response).await?;
        Ok(response
            .bytes_stream()
            .eventsource()
            .take_while(|event| {
                std::future::ready(matches!(event, Ok(event) if event.data != "[DONE]"))
            })
            .map_err(|err| anyhow::anyhow!("error parsing event: {err}"))
            .map_ok(|event| {
                serde_json::from_str(&event.data).context(format!(
                    "unable to deserialize content part: {}",
                    event.data
                ))
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
        }
    }
}
