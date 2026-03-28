use std::str::FromStr;

use mime::Mime;
use serde::{Deserializer, Serializer, de::Visitor};

pub fn serialize<S>(mime: &Mime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(mime)
}

struct Vis;

impl<'de> Visitor<'de> for Vis {
    type Value = Mime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a mime / media type such as \"application/json\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Mime::from_str(v).map_err(serde::de::Error::custom)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Mime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(Vis)
}
