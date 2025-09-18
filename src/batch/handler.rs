use snafu::ResultExt;
use tracing::Span;

use super::{
    error,
    structures::{BatchCheckResponse, BatchResponseResult},
};
use crate::client::GigaChatClient;

use super::structures::Status;

pub struct BatchHandler {
    client: GigaChatClient,
    id: String,
}

pub enum BatchCheckResult {
    Pending,
    Success { responses: Vec<BatchResponseResult> },
    InProgress { ready: usize, total: usize },
}

impl BatchHandler {
    pub fn new(client: GigaChatClient, id: String) -> Self {
        Self { client, id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    #[tracing::instrument(skip_all, fields(url))]
    async fn check_request(&self) -> Result<BatchCheckResponse, error::Error> {
        let url = self
            .client
            .build_url("batches", [("batch_id", self.id.as_str())].as_slice())
            .context(error::BuildUrlSnafu)?;
        Span::current().record("url", url.as_str());

        self.client
            .perform_request(|c| c.post(url), async |r| r.json().await)
            .await
            .context(error::BadRequestSnafu)
    }

    #[tracing::instrument(skip_all)]
    pub async fn check(&self) -> Result<BatchCheckResult, error::Error> {
        let check_response = self.check_request().await?;
        Ok(match check_response.status {
            Status::Created => BatchCheckResult::Pending,
            Status::InProgress => BatchCheckResult::InProgress {
                ready: check_response.request_counts.completed
                    + check_response.request_counts.failed,
                total: check_response.request_counts.total,
            },
            Status::Completed => todo!(),
        })
    }
}
