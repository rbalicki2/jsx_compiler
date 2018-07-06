#![feature(proc_macro, proc_macro_non_items)]

#[macro_use]
extern crate jsx_macro;
use jsx_macro::jsx;
// extern crate jsx_types;

fn main() {
  let _ = jsx!(<div></div>fofo); //~ ERROR proc macro panicked
}