use reqwest::{Certificate, ClientBuilder, Proxy, Response, Url};
use reqwest_auth::AuthorizationHeaderMiddleware;
use reqwest_middleware::ClientWithMiddleware;
use std::sync::{Arc, LazyLock};
use token_source::TokenSourceProvider;

pub mod credentials_provider;
use credentials_provider::{SberTokenProvider, SberTokenSource, TokenScope};

pub static DEFAULT_AUTH_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://ngw.devices.sberbank.ru:9443/api/v2/oauth")
        .expect("unreachable error: failed to parse default auth URL")
});

pub static DEFAULT_GIGACHAT_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://gigachat.devices.sberbank.ru/api/v1/")
        .expect("unreachable error: failed to parse default gigachat base URL")
});

pub struct GigaChatClientBuilder {
    http_client_builder: ClientBuilder,
    scope: TokenScope,
    auth_url: Url,
    gigachat_base_url: Url,
    token: String,
}

impl GigaChatClientBuilder {
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

    /// Set a proxy for the HTTP client
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.http_client_builder = self.http_client_builder.proxy(proxy);
        self
    }

    #[rustfmt::skip]
    pub async fn build(self) -> anyhow::Result<GigaChatClient> {
        let client = self.http_client_builder.build()?;
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

#[derive(Clone)]
pub struct GigaChatClient {
    pub(crate) inner: Arc<GigaChatClientInner>,
}

impl GigaChatClient {
    pub(crate) async fn check_response(response: Response) -> anyhow::Result<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let text = response.text().await?;
            Err(anyhow::anyhow!(
                "HTTP request failed with status {}; text: {text}",
                status,
            ))
        }
    }
}
