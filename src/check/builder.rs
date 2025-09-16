use snafu::{OptionExt, ResultExt};
use tracing::Span;

use super::{Model, error::*, structures::*};
use crate::client::GigaChatClient;

pub struct CheckBuilder {
    pub(crate) client: GigaChatClient,
    pub(crate) model: Model,
    pub(crate) text: Option<String>,
}

impl CheckBuilder {
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    #[tracing::instrument(skip_all, fields(url))]
    pub async fn execute(self) -> Result<CheckResponse, Error> {
        let reqwest = CheckRequest {
            input: self.text.context(TextIsMissingSnafu)?,
            model: self.model,
        };

        let url = self
            .client
            .build_url("ai/check", None)
            .context(BuildUrlSnafu)?;
        Span::current().record("url", url.as_str());
        tracing::debug!("URL constructed successfully");

        self.client
            .perform_request(|c| c.post(url).json(&reqwest), async |r| r.json().await)
            .await
            .context(BadRequestSnafu)
    }
}
