use serde::{Deserialize, Serialize};

pub mod builder;
pub mod structures;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "GigaChat-2")]
    GigaChat2Lite,
    #[serde(rename = "GigaChat-2-Pro")]
    GigaChat2Pro,
    #[default]
    #[serde(rename = "GigaChat-2-Max")]
    GigaChat2Max,
    #[serde(untagged)]
    Custom(String),
}
