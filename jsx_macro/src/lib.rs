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
  let input_2: proc_macro2::TokenStream = input.into();
  let vec_of_trees: Vec<proc_macro2::TokenTree> = input_2.into_iter().collect();

  let parsed = parsers::match_jsx(&vec_of_trees);
  let unwrapped = parsed.unwrap();
  let remaining = unwrapped.0;

  if remaining.len() > 0 {
    panic!("the jsx! macro had left over characters. Make sure you only pass one html node.");
  }

  unwrapped.1.into()
}

#[proc_macro]
pub fn jsx_verbose(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  println!("\nInput = {:?}\n", input);
  let input_2: proc_macro2::TokenStream = input.into();
  let vec_of_trees: Vec<proc_macro2::TokenTree> = input_2.into_iter().collect();

  let parsed = parsers::match_jsx(&vec_of_trees);
  println!("Output = {:?}\n", parsed);
  let unwrapped = parsed.unwrap();
  println!("Output2 = {}\n", unwrapped.1);
  let remaining = unwrapped.0;
  println!("remaining = {:?}\n", remaining);

  if remaining.len() > 0 {
    panic!("the jsx! macro had left over characters. Make sure you only pass one html node.");
  }

  unwrapped.1.into()
}
