use axum::body::Bytes;
use serde::de::DeserializeOwned;

use crate::json_multipart::{JsonMultiPart, JsonMultiPartError};

impl<T: DeserializeOwned> JsonMultiPart<T> {
    pub(super) fn try_parse(bytes: Bytes, boundary: &str) -> Result<Self, JsonMultiPartError> {
        let lossy = String::from_utf8_lossy(bytes.as_ref());
        println!("boundary = {}\n\n", boundary);
        println!("{lossy}");

        Ok(Self {
            json: serde_json::from_str("{}")
                .map_err(|err| JsonMultiPartError::Json(err.to_string().into_boxed_str()))?,
            file: Vec::new(),
        })
    }
}
