use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    check::structures::{CheckRequest, CheckResponse},
    client::GigaChatClient,
};

pub mod structures;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Model {
    GigaCheckClassification,
    #[default]
    GigaCheckDetection,
}

pub struct CheckBuilder {
    client: GigaChatClient,
    model: Option<Model>,
    text: Option<String>,
}

impl CheckBuilder {
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }

    #[tracing::instrument(skip_all, fields(url))]
    pub async fn execute(self) -> anyhow::Result<CheckResponse> {
        let reqwest = CheckRequest {
            input: self.text.context("text sould be present")?,
            model: self.model.unwrap_or_default(),
        };
        let url = self.client.inner.base_url.join("ai/check").unwrap();
        Span::current().record("url", url.as_str());
        let response = self
            .client
            .inner
            .client
            .post(url)
            .json(&reqwest)
            .send()
            .await?;

        GigaChatClient::check_response(response)
            .await?
            .json()
            .await
            .context("failed to parse check response")
    }
}

impl GigaChatClient {
    pub async fn check(&self) -> CheckBuilder {
        CheckBuilder {
            client: self.clone(),
            model: None,
            text: None,
        }
    }
}
