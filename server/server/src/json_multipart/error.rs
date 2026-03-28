use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum JsonMultiPartError {
    #[error("No Content Type")]
    HttpNoContentType,

    #[error("Content Type Is Not Valid")]
    HttpContentTypeNotValid,

    #[error("Expected MultiPart Mixed")]
    HttpExpectedMultipartMixed,

    #[error("No Boundary In Content Type")]
    HttpNoBoundary,

    #[error("{0}")]
    HttpBody(Box<str>),

    #[error("No application/json Part")]
    MultiPartNoJsonPart,

    #[error("No video Part")]
    MultiPartNoVideoPart,

    #[error("Malformed Multi Part Syntax")]
    MultiPartSyntax,

    #[error("{0}")]
    Json(Box<str>),
}
