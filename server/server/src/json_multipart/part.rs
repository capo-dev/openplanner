use std::str::FromStr;

use axum::body::Bytes;
use mime::Mime;

use crate::json_multipart::JsonMultiPartError;

use super::header::Header;

pub struct Part {
    pub content_type: Mime,
    pub body: Bytes,
}

impl Part {
    pub fn try_new(mut bytes: Bytes) -> Result<Self, JsonMultiPartError> {
        let mut content_type = None;
        while !bytes.starts_with(b"\r\n") {
            let line_idx = bytes
                .windows(2)
                .position(|window| window == b"\r\n")
                .ok_or(JsonMultiPartError::MultiPartSyntax)?;
            let header = Header::try_new(&bytes[..line_idx])?;
            if header.name.eq_ignore_ascii_case(b"content-type") {
                let parsed = Mime::from_str(header.value)
                    .map_err(|_| JsonMultiPartError::MultiPartSyntax)?;
                content_type = Some(parsed);
            }

            bytes = bytes.slice(line_idx + 2..);
        }
        bytes = bytes.slice(2..);

        match content_type {
            Some(content_type) => Ok(Self {
                content_type,
                body: bytes,
            }),
            None => Err(JsonMultiPartError::MultiPartSyntax),
        }
    }
}
