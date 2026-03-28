use proc_macro2::Span;
use serde::Deserialize;
use syn::{Error, Result};
use toml::{Table, Value};

#[derive(Deserialize)]
pub struct Proj {
    pub config: Table,
}

const CONTENTS: &str = include_str!("../../../../Proj.toml");

impl Proj {
    pub fn try_new(span: Span) -> Result<Self> {
        match toml::from_str(CONTENTS) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(span, err)),
        }
    }

    pub fn try_resolve<'a>(&'a self, key: &str, span: Span) -> Result<&'a Value> {
        match self.config.get(key) {
            Some(some) => Ok(some),
            None => Err(Error::new(span, "Key Not Found In Proj Config")),
        }
    }
}
