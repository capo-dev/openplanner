use proc_macro::TokenStream;
use syn::parse_macro_input;

mod proj;

#[proc_macro]
pub fn proj(input: TokenStream) -> TokenStream {
    match proj::handler(parse_macro_input!(input)) {
        Ok(ok) => ok,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
