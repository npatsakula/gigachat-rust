use snafu::ResultExt;
use std::io::{BufWriter, Write};
use tokio::task::spawn_blocking;
use tracing::Span;

use super::{error, handler::BatchHandler, structures::BatchCreateResponse};
use crate::{client::GigaChatClient, generation::structures::GenerationRequest};

/// Сборщик пакетных запросов.
pub struct BatchBuilder {
    pub(crate) client: GigaChatClient,
    pub(crate) requests: Vec<GenerationRequest>,
}

impl BatchBuilder {
    /// Добавляет запрос в пакет.
    pub fn with_request(mut self, request: GenerationRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// Добавляет запросы в пакет.
    pub fn with_requests(mut self, requests: Vec<GenerationRequest>) -> Self {
        self.requests.extend_from_slice(&requests);
        self
    }

    /// Создает JSONL представление пакета.
    fn serialize_batch_to_file(requests: Vec<GenerationRequest>) -> Result<Vec<u8>, error::Error> {
        let mut result = Vec::with_capacity(std::mem::size_of_val(&requests));
        let mut writer = BufWriter::new(&mut result);
        for request in requests {
            serde_json::to_writer(&mut writer, &request)
                .context(error::BatchSerializationFailedSnafu)?;
            writer
                .write_all(b"\n")
                .expect("vector write should always finish correctly");
        }
        drop(writer);
        Ok(result)
    }

    /// Выполняет пакетный запрос.
    #[tracing::instrument(skip_all, fields(
        url,
        batch.size = self.requests.len(),
        batch.bytes,
    ))]
    pub async fn execute(self) -> Result<BatchHandler, error::Error> {
        let url = self
            .client
            .build_url("batches", [("method", "chat_completions")].as_slice())
            .context(error::BuildUrlSnafu)?;
        Span::current().record("url", url.as_str());

        // [`Self::serialize_batch_to_file`] can block async runtime on large batches.
        let batch_bytes = spawn_blocking(move || Self::serialize_batch_to_file(self.requests))
            .await
            .expect("failed to join blocking thread")?;
        Span::current().record("batch.bytes", batch_bytes.len());
        tracing::debug!("batch evaluated");

        self.client
            .perform_request(
                |c| {
                    c.post(url)
                        .body(batch_bytes)
                        .header("content-type", "application/octet-stream")
                },
                async |r| r.json::<BatchCreateResponse>().await,
            )
            .await
            .context(error::BadRequestSnafu)
            .map(|r| BatchHandler::new(self.client.clone(), r.id))
    }
}
