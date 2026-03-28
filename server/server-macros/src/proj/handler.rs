use proc_macro2::TokenStream;
use syn::{LitStr, Result};

use crate::proj::{embed::embed, project::Proj};

pub fn handler(input: LitStr) -> Result<TokenStream> {
    let span = input.span();
    let proj = Proj::try_new(span)?;

    let key = input.value();
    let value = proj.try_resolve(key.as_str(), span)?;

    embed(value, span)
}
