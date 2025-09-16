use serde::{Deserialize, Serialize};

use crate::client::GigaChatClient;

pub mod builder;
pub mod error;
pub mod structures;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Model {
    GigaCheckClassification,
    #[default]
    GigaCheckDetection,
}

impl GigaChatClient {
    pub async fn check(&self) -> builder::CheckBuilder {
        builder::CheckBuilder {
            client: self.clone(),
            model: Model::default(),
            text: None,
        }
    }
}
