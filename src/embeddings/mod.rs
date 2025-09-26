use crate::client::GigaChatClient;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

pub mod error;
pub mod structures;
use structures::{EmbeddingRequest, EmbeddingResponse, Input};

/// Модель для создания векторных представлений.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Model {
    /// Базовая модель с контекстом 512 токенов.
    Embeddings,
    /// Улучшенная модель с контекстом 4096 токенов.
    #[default]
    EmbeddingsGigaR,
    /// Пользовательская модель.
    #[serde(untagged)]
    Custom(String),
}

impl Model {
    /// Возвращает модель в виде строки.
    pub fn as_str(&self) -> &str {
        match self {
            Model::Embeddings => "Embeddings",
            Model::EmbeddingsGigaR => "EmbeddingsGigaR",
            Model::Custom(s) => s,
        }
    }
}

/// Клиент для создания векторных представлений.
pub struct Embeddings {
    client: GigaChatClient,
}

impl Embeddings {
    /// Создает новый клиент для создания векторных представлений.
    pub fn new(client: GigaChatClient) -> Self {
        Self { client }
    }

    /// Создает векторные представления для входного текста(ов).
    ///
    /// # Аргументы
    ///
    /// * `input` - Строка или вектор строк для векторизации.
    /// * `model` - Модель для создания векторных представлений.
    ///
    /// # Возвращает
    ///
    /// Результат, содержащий ответ с векторными представлениями или ошибку.
    ///
    /// ## Пример
    ///
    /// ```rust,no_run
    /// use gigachat_rust::client::GigaChatClientBuilder;
    /// use gigachat_rust::embeddings::{Embeddings, Model};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = GigaChatClientBuilder::new("YOUR_TOKEN".to_string())
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///
    ///     let embeddings_client = Embeddings::new(client);
    ///
    ///     let response = embeddings_client
    ///         .create("Привет, мир!".to_string(), Some(Model::Embeddings))
    ///         .await
    ///         .unwrap();
    ///
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn create<I: Into<Input>>(
        &self,
        input: I,
        model: Option<Model>,
    ) -> Result<EmbeddingResponse, error::Error> {
        let model = model.unwrap_or_default();

        let request = EmbeddingRequest {
            model,
            input: input.into(),
        };

        let url = self
            .client
            .build_url("embeddings", None)
            .context(error::BuildUrlSnafu)?;

        self.client
            .perform_request(|c| c.post(url).json(&request), async |r| r.json().await)
            .await
            .context(error::RequestFailedSnafu)
    }
}
