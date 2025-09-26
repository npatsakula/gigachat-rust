use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("text is missing"))]
    TextIsMissing,
    #[snafu(display("text is too short, length: {length}"))]
    TextIsTooShort { length: usize },
    #[snafu(display("failed to build url"))]
    BuildUrl {
        source: crate::client::BuildUrlError,
    },
    #[snafu(display("bad request"))]
    BadRequest {
        source: crate::client::error::RequestError,
    },
}
