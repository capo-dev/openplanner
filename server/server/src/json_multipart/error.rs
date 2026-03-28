use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum JsonMultiPartError {
    #[error("No Content Type")]
    NoContentType,

    #[error("Content Type Is Not Valid")]
    ContentTypeNotValid,

    #[error("Expected MultiPart Mixed")]
    ExpectedMultipartMixed,

    #[error("No Boundary In Content Type")]
    NoBoundary,

    #[error("{0}")]
    Body(Box<str>),

    #[error("{0}")]
    Json(Box<str>),
}
