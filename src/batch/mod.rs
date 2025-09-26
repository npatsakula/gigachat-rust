use crate::client::GigaChatClient;

mod builder;
pub use builder::*;
pub mod error;
pub mod handler;
pub mod structures;

impl GigaChatClient {
    /// Cоздает сборщик пакетных запросов.
    pub fn batch(&self) -> BatchBuilder {
        BatchBuilder {
            client: self.clone(),
            requests: Vec::new(),
        }
    }
}
