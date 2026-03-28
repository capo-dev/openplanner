use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::header::CONTENT_TYPE,
};
use mime::{BOUNDARY, MULTIPART, Mime};
use serde::de::DeserializeOwned;

use crate::json_multipart::JsonMultiPartError;

pub struct JsonMultiPart<T> {
    pub json: T,
    pub file: Vec<u8>,
}

impl<T: DeserializeOwned> JsonMultiPart<T> {
    pub async fn try_new(request: Request) -> Result<Self, JsonMultiPartError> {
        let headers = request.headers();
        let mime: Mime = headers
            .get(CONTENT_TYPE)
            .ok_or(JsonMultiPartError::NoContentType)?
            .to_str()
            .map_err(|_| JsonMultiPartError::ContentTypeNotValid)?
            .parse()
            .map_err(|_| JsonMultiPartError::ContentTypeNotValid)?;

        if mime.type_() != MULTIPART || mime.subtype() != "mixed" {
            return Err(JsonMultiPartError::ExpectedMultipartMixed);
        }
        let boundary = mime
            .get_param(BOUNDARY)
            .ok_or(JsonMultiPartError::NoBoundary)?;

        let bytes = Bytes::from_request(request, &())
            .await
            .map_err(|err| JsonMultiPartError::Body(err.to_string().into_boxed_str()))?;

        Self::try_parse(bytes, boundary.as_str())
    }
}
