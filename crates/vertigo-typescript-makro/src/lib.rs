use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro::TokenStream;
// use syn::{ExprLit, Lit};

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    let call_site = Span::call_site();
    // emit_warning!(call_site, "HTML: input: {}", input.to_string());
    let result = HtmlParser::parse_stream(call_site, &input.to_string(), true);
    // emit_warning!(call_site, "HTML: output: {}", result);
    result.into()
}
