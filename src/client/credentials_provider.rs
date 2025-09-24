use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use time::OffsetDateTime;
use token_source::{TokenSource, TokenSourceProvider};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenScope {
    #[default]
    GigachatApiPers,
    GigachatApiB2B,
    GigachatApiCorp,
}

#[derive(Clone, Deserialize)]
pub struct CredentialsState {
    access_token: String,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    expires_at: OffsetDateTime,
}

pub struct SberTokenSource {
    token: String,
    client: Client,
    url: Url,
    scope: TokenScope,
    state: Mutex<CredentialsState>,
}

impl Debug for SberTokenSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CredentialsProvider")
            .field("url", &self.url)
            .finish()
    }
}

impl SberTokenSource {
    pub async fn new(
        client: Client,
        url: Url,
        scope: TokenScope,
        token: String,
    ) -> Result<Self, error::ClientError> {
        let result = Self {
            token,
            client,
            url,
            scope,
            state: Mutex::new(CredentialsState {
                access_token: String::new(),
                expires_at: OffsetDateTime::now_utc(),
            }),
        };

        let state = result.generate_new_state().await?;
        *result.state.lock().await = state;
        Ok(result)
    }

    async fn generate_new_state(&self) -> Result<CredentialsState, error::ClientError> {
        #[derive(Serialize)]
        pub struct NewStateForm {
            scope: TokenScope,
        }

        let response = self
            .client
            .post(self.url.clone())
            .bearer_auth(&self.token)
            .header("RqUID", Uuid::new_v4().to_string())
            .form(&NewStateForm { scope: self.scope })
            .send()
            .await
            .context(error::TokenGenerationFailedSnafu)?;

        let response = super::GigaChatClient::check_response(response)
            .await
            .map_err(|_| error::AuthenticationFailedSnafu.build())?;

        let mut new_state: CredentialsState = response
            .json()
            .await
            .context(error::TokenResponseParseFailedSnafu)?;
        new_state.access_token = format!("Bearer {}", new_state.access_token);
        Ok(new_state)
    }
}

#[async_trait::async_trait]
impl TokenSource for SberTokenSource {
    async fn token(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut state = self.state.lock().await;
        if state.expires_at > OffsetDateTime::now_utc() {
            return Ok(state.access_token.clone());
        }

        let new_state = self.generate_new_state().await?;

        *state = new_state.clone();
        Ok(new_state.access_token)
    }
}

#[derive(Debug)]
pub struct SberTokenProvider {
    inner: Arc<SberTokenSource>,
}

impl SberTokenProvider {
    pub fn new(inner: SberTokenSource) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl TokenSourceProvider for SberTokenProvider {
    fn token_source(&self) -> Arc<dyn TokenSource> {
        self.inner.clone()
    }
}
