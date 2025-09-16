use std::io::{BufWriter, Write};

use tokio::task::spawn_blocking;
use tracing::Span;

use crate::{
    batch::{handler::BatchHandler, structures::BatchCreateResponse},
    client::GigaChatClient,
    generation::structures::GenerationRequest,
};

pub struct BatchBuilder {
    pub(crate) client: GigaChatClient,
    pub(crate) requests: Vec<GenerationRequest>,
}

impl BatchBuilder {
    /// Add request to the batch.
    pub fn with_request(mut self, request: GenerationRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// Add requests to the batch.
    pub fn with_requests(mut self, requests: Vec<GenerationRequest>) -> Self {
        self.requests.extend_from_slice(&requests);
        self
    }

    /// Create JSONL representation of the batch.
    fn serialize_batch_to_file(requests: Vec<GenerationRequest>) -> anyhow::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(std::mem::size_of_val(&requests));
        let mut writer = BufWriter::new(&mut result);
        for request in requests {
            serde_json::to_writer(&mut writer, &request)?;
            writer.write_all(b"\n")?;
        }
        drop(writer);
        Ok(result)
    }

    #[tracing::instrument(skip_all, fields(
        url,
        batch.size = self.requests.len(),
        batch.bytes,
    ))]
    pub async fn execute(self) -> anyhow::Result<BatchHandler> {
        let mut url = self.client.inner.base_url.join("batches")?;
        url.query_pairs_mut()
            .append_pair("method", "chat_completions");
        Span::current().record("url", url.as_str());

        // [`Self::serialize_batch_to_file`] can block async runtime on large batches.
        let batch_bytes =
            spawn_blocking(move || Self::serialize_batch_to_file(self.requests)).await??;
        Span::current().record("batch.bytes", batch_bytes.len());
        tracing::debug!("batch evaluated");
        let response = self
            .client
            .inner
            .client
            .post(url)
            .body(batch_bytes)
            .header("content-type", "application/octet-stream")
            .send()
            .await?;

        let response = GigaChatClient::check_response(response)
            .await?
            .json::<BatchCreateResponse>()
            .await?;

        Ok(BatchHandler::new(self.client.clone(), response.id))
    }
}
