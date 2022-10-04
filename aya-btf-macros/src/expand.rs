use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Result, Token,
};

pub(crate) struct BtfMapArgs {
    pub(crate) name: Ident,
    pub(crate) map_type: Expr,
    pub(crate) key_type: Expr,
    pub(crate) value_type: Expr,
    pub(crate) max_entries: Expr,
    pub(crate) map_flags: Expr,
}

impl Parse for BtfMapArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;
        let map_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let key_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let value_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let max_entries = input.parse()?;
        input.parse::<Token![,]>()?;
        let map_flags = input.parse()?;

        Ok(Self {
            name,
            map_type,
            key_type,
            value_type,
            max_entries,
            map_flags,
        })
    }
}

pub(crate) fn btf_map(args: BtfMapArgs) -> Result<TokenStream> {
    let BtfMapArgs {
        name,
        map_type,
        key_type,
        value_type,
        max_entries,
        map_flags,
    } = args;

    let struct_name = Ident::new(&format!("_ty_{}", name), Span::call_site());
    let name_str = format!("{}", name);

    Ok(quote! {
        pub struct #struct_name {
            pub r#type: *const [i32; #map_type as usize],
            pub key: *const #key_type,
            pub value: *const #value_type,
            pub max_entries: *const [i32; #max_entries as usize],
            pub map_flags: *const [i32; #map_flags as usize],
        }

        #[link_section = ".maps"]
        #[export_name = #name_str]
        pub static mut #name: #struct_name = #struct_name {
            r#type: &[0i32; #map_type as usize] as *const [i32; #map_type as usize],
            key: ::core::ptr::null(),
            value: ::core::ptr::null(),
            max_entries: &[0i32; #max_entries as usize] as *const [i32; #max_entries as usize],
            map_flags: &[0i32; #map_flags as usize] as *const [i32; #map_flags as usize],
        };
    })
}
