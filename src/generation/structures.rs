use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Model;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Function,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    #[serde(flatten)]
    pub config: GenerationConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    Blacklist,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: FinishReason,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Usage {
    /// Количество токенов во входящем сообщении.
    pub prompt_tokens: u32,
    /// Количество токенов сгенерированных моделью.
    pub completion_tokens: u32,
    /// Количество ранее закэшированных токенов,
    /// которые были использованы при обработке запроса.
    /// Кэшированные токены вычитаются из общего числа
    /// оплачиваемых токенов.
    ///
    /// Модели GigaChat в течение некоторого времени
    /// сохраняют контекст запроса (историю сообщений
    /// массива messages, описание функций) с помощью
    /// кэширования токенов. Это позволяет повысить
    /// скорость ответа моделей и снизить стоимость
    /// работы с GigaChat API.
    pub precached_prompt_tokens: u32,
    /// Общее количество токенов.
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub choices: Vec<Choice>,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub created: OffsetDateTime,
    pub model: Model,
    pub usage: Usage,
}

impl GenerationResponse {
    pub fn text(&self) -> String {
        self.choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStreamPart {
    pub content: String,
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceStreamPart {
    pub delta: MessageStreamPart,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponseStream {
    model: Model,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub created: OffsetDateTime,
    pub choices: Vec<ChoiceStreamPart>,
}
