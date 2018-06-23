#![feature(proc_macro)]

extern crate jsx_types;
extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

#[proc_macro]
pub fn jsx(input: TokenStream) -> TokenStream {
  input
}
