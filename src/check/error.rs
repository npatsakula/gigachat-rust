use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    TextIsMissing,
    TextIsTooShort {
        length: usize,
    },
    BuildUrl {
        source: crate::client::BuildUrlError,
    },
    BadRequest {
        source: crate::client::error::RequestError,
    },
}
