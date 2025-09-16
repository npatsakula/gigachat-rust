use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    Client {
        source: crate::client::error::ClientError,
    },

    Generate {
        source: crate::generation::error::Error,
    },

    Batch {
        source: crate::batch::error::Error,
    },

    Check {
        source: crate::check::error::Error,
    },

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
