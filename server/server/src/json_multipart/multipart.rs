use axum::body::Bytes;

use crate::json_multipart::JsonMultiPartError;

use super::part::Part;

pub struct MultiPart<'a> {
    pub bytes: Bytes,
    pub boundary: &'a str,
}

impl<'a> MultiPart<'a> {
    pub fn new(bytes: Bytes, boundary: &'a str) -> Self {
        Self { bytes, boundary }
    }
}

impl<'a> Iterator for MultiPart<'a> {
    type Item = Result<Part, JsonMultiPartError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        if !self.bytes.starts_with(b"--") {
            self.bytes = Bytes::new();
            return Some(Err(JsonMultiPartError::MultiPartSyntax));
        }
        self.bytes = self.bytes.slice(2..);

        if !self.bytes.starts_with(self.boundary.as_bytes()) {
            self.bytes = Bytes::new();
            return Some(Err(JsonMultiPartError::MultiPartSyntax));
        }
        self.bytes = self.bytes.slice(self.boundary.len()..);

        if self.bytes.starts_with(b"--") {
            self.bytes = Bytes::new();
            return None;
        }
        if !self.bytes.starts_with(b"\r\n") {
            self.bytes = Bytes::new();
            return Some(Err(JsonMultiPartError::MultiPartSyntax));
        }
        self.bytes = self.bytes.slice(2..);

        let idx_rest_of_parts =
            match self
                .bytes
                .windows(self.boundary.len() + 4)
                .position(|window| {
                    window.starts_with(b"\r\n--") && window.ends_with(self.boundary.as_bytes())
                }) {
                Some(idx) => idx,
                None => {
                    self.bytes = Bytes::new();
                    return Some(Err(JsonMultiPartError::MultiPartSyntax));
                }
            };
        let part = self.bytes.slice(..idx_rest_of_parts);
        self.bytes = self.bytes.slice(idx_rest_of_parts + 2..);

        Some(Part::try_new(part))
    }
}
