#![feature(proc_macro, proc_macro_non_items)]

extern crate jsx_macro;
extern crate jsx_types;

use jsx_macro::jsx;
use jsx_types::HtmlToken;

fn main() {
  let a: HtmlToken = jsx!(<div />);
  println!("\nfinal output = {:?}\n", a);
}
