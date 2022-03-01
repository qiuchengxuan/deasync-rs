extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, ImplItem, TraitItem};

use quote::quote;

mod parse;
mod visit;

fn deasync_input(input: &mut parse::Item) -> TokenStream2 {
    match input {
        parse::Item::Impl(item) => {
            for inner in &mut item.items {
                if let ImplItem::Method(ref mut method) = inner {
                    method.sig.asyncness = None;
                }
            }
            visit::AsyncAwaitRemoval.remove_async_await(quote!(#item))
        }
        parse::Item::Trait(item) => {
            for inner in &mut item.items {
                if let TraitItem::Method(ref mut method) = inner {
                    method.sig.asyncness = None;
                }
            }
            visit::AsyncAwaitRemoval.remove_async_await(quote!(#item))
        }
        parse::Item::Fn(item) => {
            item.sig.asyncness = None;
            visit::AsyncAwaitRemoval.remove_async_await(quote!(#item))
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn deasync(_args: TokenStream, input: TokenStream) -> TokenStream {
    if !cfg!(feature = "bypass") {
        let mut item = parse_macro_input!(input as parse::Item);
        deasync_input(&mut item).into()
    } else {
        input
    }
}
