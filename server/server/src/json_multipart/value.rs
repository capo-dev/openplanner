use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::header::CONTENT_TYPE,
};
use mime::{APPLICATION_JSON, BOUNDARY, MULTIPART, Mime};
use serde::de::DeserializeOwned;

use crate::json_multipart::JsonMultiPartError;

use super::multipart::MultiPart;

pub struct JsonMultiPart<T> {
    pub json: T,
    pub video_content_type: Mime,
    pub video: Bytes,
}

impl<T: DeserializeOwned> JsonMultiPart<T> {
    pub async fn try_new(request: Request) -> Result<Self, JsonMultiPartError> {
        let headers = request.headers();
        let mime: Mime = headers
            .get(CONTENT_TYPE)
            .ok_or(JsonMultiPartError::HttpNoContentType)?
            .to_str()
            .map_err(|_| JsonMultiPartError::HttpContentTypeNotValid)?
            .parse()
            .map_err(|_| JsonMultiPartError::HttpContentTypeNotValid)?;

        if mime.type_() != MULTIPART || mime.subtype() != "mixed" {
            return Err(JsonMultiPartError::HttpExpectedMultipartMixed);
        }
        let boundary = mime
            .get_param(BOUNDARY)
            .ok_or(JsonMultiPartError::HttpNoBoundary)?;

        let bytes = Bytes::from_request(request, &())
            .await
            .map_err(|err| JsonMultiPartError::HttpBody(err.to_string().into_boxed_str()))?;

        let mut multipart = MultiPart::new(bytes, boundary.as_str());

        let json_part = multipart
            .next()
            .ok_or(JsonMultiPartError::MultiPartNoJsonPart)??;
        if json_part.content_type != APPLICATION_JSON {
            return Err(JsonMultiPartError::MultiPartNoJsonPart);
        }
        let json = serde_json::from_slice(&json_part.body)
            .map_err(|err| JsonMultiPartError::Json(err.to_string().into_boxed_str()))?;

        let video_part = multipart
            .next()
            .ok_or(JsonMultiPartError::MultiPartNoVideoPart)??;

        Ok(Self {
            json,
            video_content_type: video_part.content_type,
            video: video_part.body,
        })
    }
}
