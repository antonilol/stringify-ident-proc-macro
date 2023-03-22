#![no_std]

extern crate alloc;
use alloc::string::ToString;
use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn stringify_ident(stream: TokenStream) -> TokenStream {
    let mut iter = stream.into_iter();

    let Some(first) = iter.next() else {
        panic!("no tt");
    };

    let TokenTree::Ident(ident) = first else {
        panic!("first tt is not an ident");
    };

    if iter.next().is_some() {
        panic!("only 1 tt allowed");
    }

    TokenTree::Literal(Literal::string(&ident.to_string())).into()
}
