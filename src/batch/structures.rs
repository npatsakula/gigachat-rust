use crate::generation::structures::{GenerationRequest, GenerationResponse};
use crate::serialization::string_to_usize;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchGenerateRequestItem {
    #[serde(with = "string_to_usize")]
    pub key: usize,
    pub request: GenerationRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemError {
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchResponseResult {
    Result(GenerationResponse),
    Error(BatchItemError),
}

impl From<BatchResponseResult> for Result<GenerationResponse, BatchItemError> {
    fn from(result: BatchResponseResult) -> Self {
        match result {
            BatchResponseResult::Result(response) => Ok(response),
            BatchResponseResult::Error(err) => Err(err),
        }
    }
}

impl BatchResponseResult {
    pub fn res(self) -> Result<GenerationResponse, BatchItemError> {
        self.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGenerateResponseItem {
    #[serde(with = "string_to_usize")]
    pub key: usize,
    pub response: GenerationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Created,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    ChatCompletions,
    Embedder,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BatchCreateCounts {
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateResponse {
    pub id: String,
    pub method: Method,
    pub counts: BatchCreateCounts,
    pub status: Status,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BatchCheckCounts {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCheckResponse {
    pub id: String,
    pub method: Method,
    pub request_counts: BatchCheckCounts,
    pub status: Status,
    pub output_file_id: Option<String>,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub updated_at: OffsetDateTime,
}
