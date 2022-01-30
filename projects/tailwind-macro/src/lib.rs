#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]
#![doc(html_favicon_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{
    parse,
    parse::{Parse, ParseStream, Result},
    LitStr,
};
use tailwind_css::TailwindBuilder;

use self::inline::Inlined;

mod inline;

#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let Inlined { class, style } = parse(input).unwrap();
    let gen = quote! {
        (#class, #style)
    };
    gen.into()
}
