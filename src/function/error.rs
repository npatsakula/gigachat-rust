use snafu::Snafu;

use super::structures::FunctionCheckDiagnostics;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("bad request"))]
    BadRequest {
        source: crate::client::error::RequestError,
    },

    #[snafu(display("bad function; errors: {errors:?}"))]
    BadFunction {
        errors: Vec<FunctionCheckDiagnostics>,
    },
}
