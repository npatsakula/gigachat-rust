use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("failed to build url"))]
    BuildUrl {
        source: crate::client::BuildUrlError,
    },

    #[snafu(display("failed to serialize batch to jsonl"))]
    BatchSerializationFailed { source: serde_json::Error },

    #[snafu(display("bad request"))]
    BadRequest {
        source: crate::client::error::RequestError,
    },

    #[snafu(display("bad response from server"))]
    BadResponse {
        source: crate::client::CheckResponseError,
    },
}
