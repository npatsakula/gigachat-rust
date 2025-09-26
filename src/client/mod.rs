use reqwest::{Certificate, ClientBuilder, Proxy, Response, StatusCode, Url};
use reqwest_auth::AuthorizationHeaderMiddleware;
use reqwest_middleware::{ClientWithMiddleware, RequestBuilder};
use snafu::prelude::*;
use std::sync::{Arc, LazyLock};
use token_source::TokenSourceProvider;
use tracing::{Level, Span};

pub mod credentials_provider;
pub mod error;

use credentials_provider::{SberTokenProvider, SberTokenSource, TokenScope};

/// URL для аутентификации по умолчанию.
pub static DEFAULT_AUTH_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://ngw.devices.sberbank.ru:9443/api/v2/oauth")
        .expect("unreachable error: failed to parse default auth URL")
});

/// Базовый URL GigaChat по умолчанию.
pub static DEFAULT_GIGACHAT_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://gigachat.devices.sberbank.ru/api/v1/")
        .expect("unreachable error: failed to parse default gigachat base URL")
});

/// Сборщик клиента GigaChat.
///
/// ## Пример
///
/// ```rust,no_run
/// use gigachat_rust::client::GigaChatClientBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let client = GigaChatClientBuilder::new("YOUR_TOKEN".to_string())
///         .build()
///         .await
///         .unwrap();
/// }
/// ```
pub struct GigaChatClientBuilder {
    http_client_builder: ClientBuilder,
    scope: TokenScope,
    auth_url: Url,
    gigachat_base_url: Url,
    token: String,
}

impl GigaChatClientBuilder {
    /// Создает новый экземпляр сборщика клиента GigaChat.
    ///
    /// Принимает токен для аутентификации.
    pub fn new(token: String) -> Self {
        let root_certificate = Certificate::from_pem_bundle(include_bytes!(
            "../../certs/russian_trusted_root_ca_pem.crt"
        ))
        .unwrap()
        .pop()
        .unwrap();

        Self {
            http_client_builder: ClientBuilder::new().add_root_certificate(root_certificate),
            scope: TokenScope::default(),
            auth_url: DEFAULT_AUTH_URL.clone(),
            gigachat_base_url: DEFAULT_GIGACHAT_BASE_URL.clone(),
            token,
        }
    }

    /// Устанавливает прокси для HTTP клиента.
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.http_client_builder = self.http_client_builder.proxy(proxy);
        self
    }

    /// Собирает клиент GigaChat.
    #[rustfmt::skip]
    pub async fn build(self) -> Result<GigaChatClient, error::ClientError> {
        let client = self.http_client_builder.build()
            .context(error::BuildHttpClientSnafu)?;
        let ts = SberTokenSource::new(client.clone(), self.auth_url, self.scope, self.token).await?;
        let tp = SberTokenProvider::new(ts);

        let auth_middleware = AuthorizationHeaderMiddleware::from(tp.token_source());
        let http_client_builder = reqwest_middleware::ClientBuilder::new(client);

        let client = http_client_builder.with(auth_middleware).build();
        let inner = GigaChatClientInner {
            client,
            base_url: self.gigachat_base_url,
        };

        Ok(GigaChatClient {
            inner: Arc::new(inner),
        })
    }
}

pub(crate) struct GigaChatClientInner {
    pub(crate) client: ClientWithMiddleware,
    pub(crate) base_url: Url,
}

/// Клиент GigaChat.
#[derive(Clone)]
pub struct GigaChatClient {
    inner: Arc<GigaChatClientInner>,
}

/// Ошибка некорректного ответа.
#[derive(Debug, Snafu)]
#[snafu(display("bad response; status code {status_code}; description '{description}'"))]
pub struct CheckResponseError {
    status_code: StatusCode,
    description: String,
}

/// Ошибка сборки URL.
#[derive(Debug, Snafu)]
pub struct BuildUrlError {
    source: url::ParseError,
    base_url: Url,
    path: String,
}

impl GigaChatClient {
    #[tracing::instrument(skip_all, fields(
        url.base = self.inner.base_url.as_str(),
        url.path = path,
        url.queries,
    ), err, ret(level = Level::DEBUG))]
    pub(crate) fn build_url<'q, Q: Into<Option<&'q [(&'q str, &'q str)]>>>(
        &self,
        path: &str,
        queries: Q,
    ) -> Result<Url, BuildUrlError> {
        let mut url = self
            .inner
            .base_url
            .join(path)
            .with_context(|_| BuildUrlSnafu {
                base_url: self.inner.base_url.clone(),
                path: path.to_string(),
            })?;

        if let Some(queries) = queries.into() {
            for (key, value) in queries {
                url.query_pairs_mut().append_pair(key, value);
            }

            Span::current().record("url.queries", format!("{queries:?}"));
        }

        Ok(url)
    }

    #[tracing::instrument(skip_all, err)]
    pub(crate) async fn perform_request<
        B: FnOnce(&ClientWithMiddleware) -> RequestBuilder,
        D: AsyncFn(reqwest::Response) -> Result<T, reqwest::Error>,
        T,
    >(
        &self,
        builder: B,
        deserializer: D,
    ) -> Result<T, error::RequestError> {
        let request = builder(&self.inner.client);
        let response = request.send().await.context(error::SendRequestSnafu)?;
        tracing::debug!("request result received");
        let response = Self::check_response(response).await?;
        tracing::debug!("request result checked");
        deserializer(response)
            .await
            .context(error::ParseResponseSnafu)
    }

    #[tracing::instrument(skip_all, err)]
    async fn check_response(response: Response) -> Result<Response, error::RequestError> {
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let text = response.text().await.unwrap_or_default();
            error::BadResponseSnafu {
                status_code: status,
                description: text,
            }
            .fail()
        }
    }
}
