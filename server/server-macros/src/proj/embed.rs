use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Error, LitBool, LitFloat, LitInt, LitStr, Result};
use toml::Value;

pub fn embed(value: &Value, span: Span) -> Result<TokenStream> {
    match value {
        Value::Boolean(bool) => {
            let lit = LitBool::new(*bool, span);
            Ok(lit.into_token_stream())
        }
        Value::Integer(int) => {
            let lit = LitInt::new(int.to_string().as_str(), span);
            Ok(lit.into_token_stream())
        }
        Value::Float(flt) => {
            let lit = LitFloat::new(flt.to_string().as_str(), span);
            Ok(lit.into_token_stream())
        }
        Value::String(s) => {
            let lit = LitStr::new(s.as_str(), span);
            Ok(lit.into_token_stream())
        }
        _ => Err(Error::new(span, "Cannot Embed Complex Toml Type")),
    }
}
