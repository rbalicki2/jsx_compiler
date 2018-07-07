#![feature(proc_macro, proc_macro_non_items)]

extern crate compiletest_rs as compiletest;

extern crate jsx_macro;
extern crate jsx_types;

use jsx_macro::{jsx, jsx_verbose};
use jsx_types::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
  use super::*;

  fn get_bare_div() -> HtmlToken {
    HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![],
      attributes: HashMap::new(),
    })
  }

  #[test]
  fn basic_test() {
    let dom = jsx!(<div />);
    assert_eq!(dom, get_bare_div());
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
    let inner_dom = jsx!(<span />);
    let inner_dom_copy = jsx!(<span />);
    let dom = jsx!(<div>{ inner_dom }</div>);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        inner_dom_copy,
      ],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn child_component_interpolated_string_test() {
    // N.B. we are cloning because foo is moved in the jsx! call.
    // Is this necessary?
    let foo: String = "foo".into();
    let foo2 = foo.clone();
    let dom = jsx!(<div>{foo}</div>);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::Text(foo2.into()),
      ],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn child_component_interpolated_str_test() {
    // N.B. we are cloning because foo is moved in the jsx! call.
    // Is this necessary?
    let foo: &str = "foo";
    let foo2 = foo.clone();
    let dom = jsx!(<div>{foo}</div>);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::Text(foo2.into()),
      ],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn non_self_closing_component() {
    let dom = jsx!(<div></div>);
    assert_eq!(dom, get_bare_div());
  }

  #[test]
  fn strings_are_valid_jsx() {
    let dom = jsx!(foo);
    assert_eq!(dom, HtmlToken::Text("foo".into()));
  }

  #[test]
  fn multiple_strings_are_valid_jsx() {
    let dom = jsx!(foo bar);
    assert_eq!(dom, HtmlToken::Text("foo bar".into()));
  }

  #[test]
  fn many_spaces_are_valid_jsx() {
    let dom = jsx!(foo  bar   baz    qux);
    assert_eq!(dom, HtmlToken::Text("foo  bar   baz    qux".into()));
  }

  #[test]
  fn multi_line_spaces_work_correctly() {
    // N.B. this is weird behavior, and should be changed, but for now,
    // let's document it.
    let dom = jsx!(<div>foo
      bar
    </div>);
    assert_eq!(dom, HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![HtmlToken::Text("foobar".into())],
      attributes: HashMap::new(),
    }));
  }

  #[test]
  fn multiple_strings_are_valid_jsx_2() {
    let dom = jsx!(foo bar "baz" 'q' ux);
    // N.B. we include the quotes, which is ... correct?
    // TODO characters should probably not have single quotes around it.
    // because that's how we'd include parentheses, backslash, <, etc.
    assert_eq!(dom, HtmlToken::Text("foo bar \"baz\" \'q\' ux".into()));
  }

  #[test]
  fn random_characters_allowed_in_strings() {
    // N.B. obviously < is not allowed
    // Also, neither is a backslash, apparently.
    let dom = jsx!(+ / * ^ #@&);
    assert_eq!(dom, HtmlToken::Text("+ / * ^ #@&".into()));
  }

  #[test]
  fn interpolated_strings_by_themselves_are_valid_jsx() {
    let bar = "bar";
    let dom = jsx!({ bar });
    assert_eq!(dom, HtmlToken::Text(bar.into()));
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
