use std::io::{BufWriter, Write};

use crate::{
    batch::{handler::BatchHandler, structures::BatchCreateResponse},
    client::GigaChatClient,
    generation::structures::GenerationRequest,
};

pub mod handler;
pub mod structures;

pub struct BatchBuilder {
    client: GigaChatClient,
    requests: Vec<GenerationRequest>,
}

impl BatchBuilder {
    pub fn with_request(mut self, request: GenerationRequest) -> Self {
        self.requests.push(request);
        self
    }

    pub fn with_requests(mut self, requests: Vec<GenerationRequest>) -> Self {
        self.requests = requests;
        self
    }

    fn transform_batch(&self) -> anyhow::Result<Vec<u8>> {
        let mut result = Vec::new();
        let mut writer = BufWriter::new(&mut result);
        for request in &self.requests {
            serde_json::to_writer(&mut writer, &request)?;
            writer.write_all(b"\n")?;
        }
        drop(writer);
        Ok(result)
    }

    pub async fn execute(&self) -> anyhow::Result<BatchHandler> {
        let mut url = self.client.inner.base_url.join("batches")?;
        url.query_pairs_mut()
            .append_pair("method", "chat_completions");

        let batch_bytes = self.transform_batch()?;
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

impl GigaChatClient {
    pub fn batch(&self) -> BatchBuilder {
        BatchBuilder {
            client: self.clone(),
            requests: Vec::new(),
        }
    }
}
