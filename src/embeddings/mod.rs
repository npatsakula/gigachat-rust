use crate::client::GigaChatClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod structures;
use structures::{EmbeddingRequest, EmbeddingResponse, Input};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Model {
    Embeddings,
    #[default]
    EmbeddingsGigaR,
    #[serde(untagged)]
    Custom(String),
}

impl Model {
    pub fn as_str(&self) -> &str {
        match self {
            Model::Embeddings => "Embeddings",
            Model::EmbeddingsGigaR => "EmbeddingsGigaR",
            Model::Custom(s) => s,
        }
    }
}

pub struct Embeddings {
    client: GigaChatClient,
}

impl Embeddings {
    pub fn new(client: GigaChatClient) -> Self {
        Self { client }
    }

    /// Creates embeddings for the input text(s)
    ///
    /// # Arguments
    ///
    /// * `input` - A string or vector of strings to embed
    /// * `model` - The model to use for embeddings (optional, defaults to EmbeddingsGigaR)
    ///
    /// # Returns
    ///
    /// A result containing the embedding response or an error
    pub async fn create<I: Into<Input>>(
        &self,
        input: I,
        model: Option<Model>,
    ) -> Result<EmbeddingResponse> {
        let model = model.unwrap_or_default();

        let request = EmbeddingRequest {
            model,
            input: input.into(),
        };

        let url = self.client.inner.base_url.join("embeddings")?;

        let response = self
            .client
            .inner
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send embeddings request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Embeddings API error {}: {}",
                status,
                error_text
            ));
        }

        let embedding_response: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse embeddings response: {}", e))?;

        Ok(embedding_response)
    }
}
