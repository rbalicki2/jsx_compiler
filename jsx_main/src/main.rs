#![feature(proc_macro, proc_macro_non_items)]

extern crate jsx_macro;

use jsx_macro::jsx;

fn main() {
  let a = jsx!("123");
  println!("{}", a);
}
