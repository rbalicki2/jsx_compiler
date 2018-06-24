#![feature(proc_macro)]

extern crate jsx_types;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quote;

mod parsers;

#[proc_macro]
pub fn jsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  println!("\nInput = {:?}\n", input);
  let input_2: proc_macro2::TokenStream = input.into();
  let vec_of_trees: Vec<proc_macro2::TokenTree> = input_2.into_iter().collect();

  let parsed = parsers::match_html_token(&vec_of_trees);
  println!("Output = {:?}\n", parsed);
  let unwrapped = parsed.unwrap().1;
  println!("Output2 = {}\n", unwrapped);

  unwrapped.into()
}
