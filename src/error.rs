use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("client error"))]
    Client {
        source: crate::client::error::ClientError,
    },

    #[snafu(display("function error"))]
    Function {
        source: crate::function::error::Error,
    },

    #[snafu(display("generation error"))]
    Generate {
        source: crate::generation::error::Error,
    },

    #[snafu(display("batch error"))]
    Batch { source: crate::batch::error::Error },

    #[snafu(display("check error"))]
    Check { source: crate::check::error::Error },

    #[snafu(display("embeddings error"))]
    Embeddings {
        source: crate::embeddings::error::Error,
    },

    #[snafu(whatever, display("{message}"))]
    Custom {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
