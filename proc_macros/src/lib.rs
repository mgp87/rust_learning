extern crate proc_macro; // used to import proc_macro crate needed to define and implement macros

use proc_macro::TokenStream; // types and functions for working with procedural macros
// token stream is a type that represents a sequence of tokens that's used as the output of a procedural macro
// a proc macro takes some input code, manipulates it in some way and returns a TokenStream representing the modified code

use quote::quote; // quote is a macro that can generate a TokenStream from a syntax tree
// it allows to construct rust code using rust syntax

use syn::{parse_macro_input, ItemFn};
// syn crate is a third party crate that provides a parser for rust syntax
// it can parse rust code into a syntax tree that can be manipulated by a procedural macro

// syn is going to put it into a syntax tree where quote is going to take it from that syntax tree
// and turn it into a TokenStream for us

#[proc_macro_attribute]
pub fn debug_print(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // item contains the actual rust code that's going to be modified by our macro here
    let mut item_fn = parse_macro_input!(item as ItemFn);

    let ident = &item_fn.sig.ident; // returns a reference to the item function containing all metadata, ident returns a reference to the function's name

    item_fn.block.stmts.insert(
        0, // index
        syn::parse_quote!(println!("Entering function: {}", stringify!(#ident));)
    );

    TokenStream::from(quote!{
        #item_fn
    })
}