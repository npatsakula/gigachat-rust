use serde::{Deserialize, Serialize};

use crate::{
    check::structures::{CheckRequest, CheckResponse},
    client::GigaChatClient,
};

pub mod structures;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Model {
    GigaCheckClassification,
    #[default]
    GigaCheckDetection,
}

impl GigaChatClient {
    pub async fn check(&self, model: Model, text: String) -> anyhow::Result<CheckResponse> {
        let reqwest = CheckRequest { input: text, model };
        let url = self.inner.base_url.join("ai/check").unwrap();
        println!("{url}");
        let response = self.inner.client.post(url).json(&reqwest).send().await?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Request failed with status {} and text: {}",
                status,
                text
            ));
        }

        let response = response.json::<CheckResponse>().await?;

        Ok(response)
    }
}
