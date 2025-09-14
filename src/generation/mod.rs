use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    client::GigaChatClient,
    generation::structures::{GenerationRequest, GenerationResponse, Message},
};

pub mod structures;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "GigaChat-2")]
    GigaChat2Lite,
    #[serde(rename = "GigaChat-2-Pro")]
    GigaChat2Pro,
    #[default]
    #[serde(rename = "GigaChat-2-Max")]
    GigaChat2Max,
    #[serde(untagged)]
    Custom(String),
}

pub struct GenerationBuilder {
    client: GigaChatClient,

    model: Model,
    messages: Option<Vec<Message>>,
}

impl GenerationBuilder {
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = Some(messages);
        self
    }

    pub async fn execute(&self) -> anyhow::Result<GenerationResponse> {
        let request = GenerationRequest {
            model: self.model.clone(),
            messages: self.messages.clone().unwrap_or_default(),
            config: Default::default(),
        };

        let url = self.client.inner.base_url.join("chat/completions")?;
        println!("{url}");
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

        println!("{result}");

        Ok(serde_json::from_value(result)?)
    }
}

impl GigaChatClient {
    pub fn generate(&self) -> GenerationBuilder {
        GenerationBuilder {
            client: self.clone(),
            model: Model::default(),
            messages: None,
        }
    }
}
