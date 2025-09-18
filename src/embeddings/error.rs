use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    RequestFailed {
        source: crate::client::error::RequestError,
    },

    #[snafu(display("failed to build url"))]
    BuildUrl {
        source: crate::client::BuildUrlError,
    },
}
