use derive_more::From;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub model: Model,
    pub input: Input,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Usage {
    /// Количество токенов в строке, для которой сгенерирован эмбеддинг.
    pub prompt_tokens: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingResponseItem {
    pub embedding: Vec<f32>,
    pub index: u64,
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub model: Model,
    pub data: Vec<EmbeddingResponseItem>,
}
