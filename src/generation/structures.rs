use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::function::{FunctionName, UserFunction};

use super::Model;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallResponse {
    #[serde(flatten)]
    pub name: FunctionName,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    System {
        content: String,
    },
    User {
        content: String,
    },
    Assistant {
        content: String,
        function_call: Option<FunctionCallResponse>,
    },
    Function {
        #[serde(flatten)]
        name: FunctionName,
        #[serde(with = "crate::serialization::string_json")]
        content: serde_json::Value,
    },
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self::System {
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::User {
            content: content.into(),
        }
    }

    pub fn function<C: Serialize>(name: FunctionName, content: C) -> Self {
        Self::Function {
            name,
            content: serde_json::to_value(content).unwrap(),
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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionCall {
    #[default]
    None,
    Auto,
    #[serde(untagged)]
    Manual(FunctionName),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Function {
    User(UserFunction),
    BuiltIn(FunctionName),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    pub function_call: FunctionCall,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<Function>,
    #[serde(flatten)]
    pub config: GenerationConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    FunctionCall,
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
    #[serde(with = "time::serde::timestamp")]
    pub created: OffsetDateTime,
    pub model: Model,
    pub usage: Usage,
}

impl GenerationResponse {
    pub fn text(&self) -> String {
        self.choices
            .first()
            .and_then(|choice| match &choice.message {
                Message::System { content }
                | Message::User { content }
                | Message::Assistant { content, .. } => Some(content.clone()),
                Message::Function { .. } => None,
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageStreamPart {
    Header(Message),
    Delta { content: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceStreamPart {
    pub delta: MessageStreamPart,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponseStream {
    model: Model,
    #[serde(with = "time::serde::timestamp")]
    pub created: OffsetDateTime,
    pub choices: Vec<ChoiceStreamPart>,
}
