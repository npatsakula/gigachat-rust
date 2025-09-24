use eventsource_stream::EventStreamError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("failed to parse generation response"))]
    ResponseParseFailed { source: reqwest::Error },

    BadRequest {
        source: crate::client::error::RequestError,
    },

    #[snafu(display("stream connection error"))]
    StreamConnectionFailed { source: reqwest_middleware::Error },

    #[snafu(display("event stream parsing error"))]
    EventParseFailed {
        source: EventStreamError<reqwest::Error>,
    },

    #[snafu(display("failed to deserialize stream event: {event_data}"))]
    StreamDeserializationFailed {
        event_data: String,
        source: serde_json::Error,
    },

    #[snafu(display("stream ended unexpectedly"))]
    StreamEndedUnexpectedly,

    #[snafu(display("failed to build url"))]
    BuildUrl {
        source: crate::client::BuildUrlError,
    },
}
