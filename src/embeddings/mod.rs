use crate::client::GigaChatClient;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

pub mod error;
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
    ) -> Result<EmbeddingResponse, error::Error> {
        let model = model.unwrap_or_default();

        let request = EmbeddingRequest {
            model,
            input: input.into(),
        };

        let url = self
            .client
            .build_url("embeddings", None)
            .context(error::BuildUrlSnafu)?;

        self.client
            .perform_request(|c| c.post(url).json(&request), async |r| r.json().await)
            .await
            .context(error::RequestFailedSnafu)
    }
}
