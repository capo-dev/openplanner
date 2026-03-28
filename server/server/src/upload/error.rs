use std::path::PathBuf;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use crate::json_multipart::JsonMultiPartError;

#[derive(Debug, Error, Serialize)]
pub enum UploadError {
    #[error(transparent)]
    JsonMultiPart(#[from] JsonMultiPartError),

    #[error("Failed To Get Current Working Directory: {0}")]
    Cwd(Box<str>),

    #[error("{}: {err}", .path.display())]
    Io { path: PathBuf, err: Box<str> },

    #[error("{0}")]
    Json(Box<str>),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}
