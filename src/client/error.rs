use snafu::Snafu;
use url::ParseError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ClientError {
    #[snafu(display("failed to build http client"))]
    BuildHttpClient { source: reqwest::Error },

    #[snafu(display("failed to parse url"))]
    UrlParse { source: ParseError },

    #[snafu(display("authentication failed"))]
    AuthenticationFailed,

    #[snafu(display("token generation failed"))]
    TokenGenerationFailed { source: reqwest::Error },

    #[snafu(display("failed to parse token response"))]
    TokenResponseParseFailed { source: reqwest::Error },

    #[snafu(display("certificate error"))]
    Certificate { source: reqwest::Error },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum RequestError {
    #[snafu(display("failed to send request"))]
    SendRequest { source: reqwest_middleware::Error },

    BadResponse {
        status_code: u16,
        description: String,
    },

    #[snafu(display("failed to parse response"))]
    ParseResponse { source: reqwest::Error },
}
