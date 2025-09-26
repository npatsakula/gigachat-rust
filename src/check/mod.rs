use serde::{Deserialize, Serialize};

use crate::client::GigaChatClient;

pub mod builder;
pub mod error;
pub mod structures;

/// Модель для проверки текста.
///
/// Используется для определения, был ли текст сгенерирован нейросетевыми моделями.
///
/// ## Примечание
///
/// Проверка доступна только для текстов на русском языке длиной не менее 20 слов.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Model {
    /// Возвращает вероятность того, что текст написан человеком.
    GigaCheckClassification,
    /// Возвращает интервалы текста, которые с большой вероятностью сгенерированы ИИ.
    #[default]
    GigaCheckDetection,
}

impl GigaChatClient {
    /// Cоздает сборщик запроса проверки текста.
    ///
    /// ## Пример
    ///
    /// ```rust,no_run
    /// use gigachat_rust::client::GigaChatClientBuilder;
    /// use gigachat_rust::check::Model;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = GigaChatClientBuilder::new("YOUR_TOKEN".to_string())
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///
    ///     let response = client
    ///         .check()
    ///         .await
    ///         .with_text("Этот текст был написан человеком, а не искусственным интеллектом. Он содержит более двадцати слов, чтобы соответствовать требованиям API.".to_string())
    ///         .with_model(Model::GigaCheckClassification)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    ///
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn check(&self) -> builder::CheckBuilder {
        builder::CheckBuilder {
            client: self.clone(),
            model: Model::default(),
            text: None,
        }
    }
}
