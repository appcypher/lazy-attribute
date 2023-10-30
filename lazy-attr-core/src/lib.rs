#![warn(missing_docs)]
#![doc = include_str!("lib.md")]

mod lazy;

use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

/// TODO(appcypher): document
#[proc_macro_attribute]
pub fn lazy(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(fn_syntax).into()
}
