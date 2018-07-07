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

  #[test]
  fn basic_test() {
    let dom = jsx!(<div />);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn attribute_test() {
    let qux = "qux";
    let dom = jsx!(<div foo="bar" baz={qux} />);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![],
      attributes: {
        let mut map = HashMap::new();
        map.insert("foo".into(), "bar".into());
        map.insert("baz".into(), qux.into());
        map
      },
    }));
  }

  #[test]
  fn child_component_dom_element_test() {
    let dom = jsx!(<div><h1 /></div>);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::DomElement(DomElement {
          node_type: "h1".into(),
          children: vec![],
          attributes: HashMap::new(),
        }),
      ],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn child_component_string_test() {
    let dom = jsx!(<div>foo bar</div>);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::Text("foo bar".into()),
      ],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn child_component_interpolated_test() {
    // let inner_dom = jsx!(<span />);
    // let dom = jsx!(<div>{ inner_dom }</div>);
    // assert_eq!(dom, HtmlToken::DomElement(DomElement {
    //   node_type: "div".into(),
    //   children: vec![
    //     inner_dom,
    //   ],
    //   attributes: HashMap::new(),
    // }));
  }

  #[test]
  fn non_self_closing_component() {

  }

  #[test]
  fn strings_are_valid_jsx() {

  }

  #[test]
  fn interpolated_jsx_is_valid() {

  }

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
