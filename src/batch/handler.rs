use crate::{
    batch::structures::{BatchCheckResponse, BatchResponseResult},
    client::GigaChatClient,
};

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

    async fn check_request(&self) -> anyhow::Result<BatchCheckResponse> {
        let mut url = self.client.inner.base_url.join("batches")?;
        url.query_pairs_mut().append_pair("batch_id", &self.id);

        let response = self.client.inner.client.post(url).send().await?;
        Ok(GigaChatClient::check_response(response)
            .await?
            .json()
            .await?)
    }

    pub async fn check(&self) -> anyhow::Result<BatchCheckResult> {
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
