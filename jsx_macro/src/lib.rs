#![feature(proc_macro)]

extern crate jsx_types;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quote;

mod parsers;

use proc_macro::{TokenStream,TokenTree};

#[proc_macro]
pub fn jsx(input: TokenStream) -> TokenStream {
  println!("\nInput = {:?}\n", input);
  let vec_of_trees: Vec<TokenTree> = input.into_iter().collect();

  let parsed = parsers::match_html_token(&vec_of_trees);
  println!("Output = {:?}\n", parsed);

  parsed.unwrap().1
}
