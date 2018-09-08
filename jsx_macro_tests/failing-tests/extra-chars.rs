#![feature(proc_macro_non_items)]

#[macro_use]
extern crate jsx_macro;
use jsx_macro::jsx;

fn main() {
  let _ = jsx!(<div></div>fofo); //~ ERROR proc macro panicked
}
