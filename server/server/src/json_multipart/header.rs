use crate::json_multipart::JsonMultiPartError;

pub struct Header<'a> {
    pub name: &'a [u8],
    pub value: &'a str,
}

impl<'a> Header<'a> {
    pub fn try_new(bytes: &'a [u8]) -> Result<Self, JsonMultiPartError> {
        let idx = bytes
            .iter()
            .position(|ch| *ch == b':')
            .ok_or(JsonMultiPartError::MultiPartSyntax)?;
        let name = &bytes[..idx];
        let value = &bytes[idx + 1..];
        let value = str::from_utf8(value).map_err(|_| JsonMultiPartError::MultiPartSyntax)?;
        let value = value.trim();

        Ok(Self { name, value })
    }
}
