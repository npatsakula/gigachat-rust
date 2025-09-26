use serde::{Deserialize, Serialize};

pub mod builder;
pub mod error;
pub mod structures;

/// Модель для генерации текста.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Model {
    /// Базовая модель, подходит для простых повседневных задач, требующих максимальной скорости и минимального потребления ресурсов.
    #[serde(rename = "GigaChat-2")]
    GigaChat2Lite,
    /// Улучшенная модель, предназначенная для более ресурсоемких задач, где важны креативность и точность.
    #[serde(rename = "GigaChat-2-Pro")]
    GigaChat2Pro,
    /// Самая мощная и продвинутая модель в линейке, предназначенная для самых сложных и масштабных задач, требующих высочайшего уровня креативности и качества.
    #[default]
    #[serde(rename = "GigaChat-2-Max")]
    GigaChat2Max,
    /// Пользовательская модель.
    #[serde(untagged)]
    Custom(String),
}
