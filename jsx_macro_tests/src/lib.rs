#![feature(proc_macro, proc_macro_non_items)]

extern crate compiletest_rs as compiletest;

extern crate jsx_macro;
extern crate jsx_types;

use jsx_macro::jsx;
use jsx_types::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn basic_test() {
  //   let dom = jsx!(<div />);
  //   assert_eq!(dom, HtmlToken::DomElement(DomElement {
  //     node_type: "div".into(),
  //     children: vec![],
  //     attributes: HashMap::new(),
  //   }));
  // }

  #[test]
  fn failing_test() {
    let mut config = compiletest::Config::default();
    config.mode = "compile-fail".parse().unwrap();
    config.src_base = PathBuf::from("failing-tests");
    config.link_deps();
    config.clean_rmeta();
    compiletest::run_tests(&config);
  }
}
