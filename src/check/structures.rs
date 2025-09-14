use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Текст сгенерирован с помощью нейросетевых моделей.
    Ai,
    /// Текст написан человеком.
    Human,
    /// Текст содержит как фрагменты сгенерированные с помощью моделей,
    /// так и написанные человеком.
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRequest {
    /// Текст, который будет проверен на наличие содержимого,
    /// сгенерированного с помощью нейросетевых моделей.
    ///
    /// Проверка доступна только для текстов на русском языке.
    /// Минимальная длина текста — 20 слов.
    pub input: String,
    /// Название модели.
    pub model: Model,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResponse {
    /// Результат проверки текста.
    pub category: Category,
    /// Количество символов в переданном тексте.
    pub characters: usize,
    /// Количество токенов в переданном тексте.
    pub tokens: usize,
    /// Части текста, сгенерированные моделью.
    ///
    /// Обозначаются индексами символов, с которых начинаются и
    /// заканчиваются сгенерированные фрагменты.
    pub ai_intervals: Vec<(usize, usize)>,
}
