use proc_macro::TokenStream;
use syn::parse_macro_input;

mod expand;

#[proc_macro]
pub fn btf_map(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as expand::BtfMapArgs);
    expand::btf_map(args).unwrap_or_else(|e| e.to_compile_error()).into()
}
