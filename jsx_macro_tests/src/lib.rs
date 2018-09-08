#![feature(proc_macro_non_items)]

extern crate compiletest_rs as compiletest;

extern crate jsx_macro;
extern crate jsx_types;

#[allow(unused_imports)]
use jsx_macro::{jsx, jsx_verbose};

use jsx_types::{*, events::*};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Keys;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
  use super::*;

  fn get_bare_div<'a>() -> HtmlToken<'a> {
    HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })
  }

  #[derive(PartialEq, Debug, Clone)]
  enum ComparableHtmlToken {
    Text(String),
    DomElement(ComparableDomElement),
  }

  #[derive(Debug, PartialEq, Clone)]
  struct ComparableDomElement {
    pub node_type: String,
    pub children: Vec<ComparableHtmlToken>,
    pub attributes: jsx_types::Attributes,
  }

  impl ComparableDomElement {
    fn from_dom_element(d: &jsx_types::DomElement) -> Self {
      ComparableDomElement {
        node_type: d.node_type.clone(),
        children: d.children.iter().map(|c| {
          match c {
            jsx_types::HtmlToken::Text(t) => ComparableHtmlToken::Text(t.to_string()),
            jsx_types::HtmlToken::DomElement(d) => ComparableHtmlToken::DomElement(
              ComparableDomElement::from_dom_element(&d)
            ),
          }
        }).collect(),
        attributes: d.attributes.clone(),
      }
    }
  }

  // type KeyType<'a> = Keys<'a, jsx_types::events::EventName, Box<jsx_types::EventHandler<'a>>>;
  // fn compare_event_handler_keys(k1: KeyType, k2: KeyType) -> bool {
  //   let l1 = k1.len();
  //   let s1 = k1.fold(HashSet::with_capacity(l1), |mut set, key| {
  //     set.insert(key);
  //     set
  //   });
  //   let l2 = k2.len();
  //   let s2 = k2.fold(HashSet::with_capacity(l2), |mut set, key| {
  //     set.insert(key);
  //     set
  //   });
  //   s1 == s2
  // }

  fn equal_enough(t1: &HtmlToken, t2: &HtmlToken) -> bool {
    let print_error = || println!("equal_enough returned false.\nleft={:?}\nright={:?}", t1, t2);
    match (&t1, &t2) {
      (HtmlToken::Text(s1), HtmlToken::Text(s2)) => s1 == s2,
      (HtmlToken::DomElement(d1), HtmlToken::DomElement(d2)) => {
        // N.B. this doesn't check children's event handlers...

        // let event_handlers_equal = compare_event_handler_keys(d1.event_handlers.keys(), d2.event_handlers.keys());
        // true

        // if !event_handlers_equal {
        //   print_error();
        //   return false;
        // }

        let w1: ComparableDomElement = ComparableDomElement::from_dom_element(&d1);
        let w2: ComparableDomElement = ComparableDomElement::from_dom_element(&d2);

        let result = w1 == w2;

        if !result { print_error(); }
        result
      },
      _ => { print_error(); false },
    }
  }

  #[test]
  fn basic_test() {
    let dom = jsx!(<div />);
    assert!(equal_enough(&dom, &get_bare_div()));
  }

  #[test]
  fn attribute_test() {
    let qux = "qux";
    let dom = jsx!(<div foo="bar" baz={qux} />);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![],
      attributes: {
        let mut map = HashMap::new();
        map.insert("foo".into(), "bar".into());
        map.insert("baz".into(), qux.into());
        map
      },
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn child_component_dom_element_test() {
    let dom = jsx!(<div><h1 /></div>);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::DomElement(DomElement {
          node_type: "h1".into(),
          children: vec![],
          attributes: HashMap::new(),
          event_handlers: EventHandlers::new(),
        }),
      ],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn child_component_string_test() {
    let dom = jsx!(<div>foo bar</div>);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::Text("foo bar".into()),
      ],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn child_component_interpolated_test() {
    let inner_dom = jsx!(<span />);
    let inner_dom_copy = jsx!(<span />);
    let dom = jsx!(<div>{ inner_dom }</div>);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        inner_dom_copy,
      ],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn child_component_interpolated_string_test() {
    // N.B. we are cloning because foo is moved in the jsx! call.
    // Is this necessary?
    let foo: String = "foo".into();
    let foo2 = foo.clone();
    let dom = jsx!(<div>{foo}</div>);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::Text(foo2.into()),
      ],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn child_component_interpolated_str_test() {
    // N.B. we are cloning because foo is moved in the jsx! call.
    // Is this necessary?
    let foo: &str = "foo";
    let foo2 = foo.clone();
    let dom = jsx!(<div>{foo}</div>);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![
        HtmlToken::Text(foo2.into()),
      ],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn non_self_closing_component() {
    let dom = jsx!(<div></div>);
    assert!(equal_enough(&dom, &get_bare_div()));
  }

  #[test]
  fn strings_are_valid_jsx() {
    let dom = jsx!(foo);
    assert!(equal_enough(&dom, &HtmlToken::Text("foo".into())));
  }

  #[test]
  fn multiple_strings_are_valid_jsx() {
    let dom = jsx!(foo bar);
    assert!(equal_enough(&dom, &HtmlToken::Text("foo bar".into())));
  }

  #[test]
  fn many_spaces_are_valid_jsx() {
    let dom = jsx!(foo  bar   baz    qux);
    assert!(equal_enough(&dom, &HtmlToken::Text("foo  bar   baz    qux".into())));
  }

  #[test]
  fn multi_line_spaces_work_correctly() {
    // N.B. this is weird behavior, and should be changed, but for now,
    // let's document it.
    let dom = jsx!(<div>foo
      bar
    </div>);
    assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
      node_type: "div".into(),
      children: vec![HtmlToken::Text("foobar".into())],
      attributes: HashMap::new(),
      event_handlers: EventHandlers::new(),
    })));
  }

  #[test]
  fn multiple_strings_are_valid_jsx_2() {
    let dom = jsx!(foo bar "baz" 'q' ux);
    // N.B. we include the quotes, which is ... correct?
    // TODO characters should probably not have single quotes around it.
    // because that's how we'd include parentheses, backslash, <, etc.
    assert!(equal_enough(&dom, &HtmlToken::Text("foo bar \"baz\" \'q\' ux".into())));
  }

  #[test]
  fn random_characters_allowed_in_strings() {
    // N.B. obviously < is not allowed
    // Also, neither is a backslash, apparently.
    let dom = jsx!(+ / * ^ #@&);
    assert!(equal_enough(&dom, &HtmlToken::Text("+ / * ^ #@&".into())));
  }

  #[test]
  fn interpolated_strings_by_themselves_are_valid_jsx() {
    let bar = "bar";
    let dom = jsx!({ bar });
    assert!(equal_enough(&dom, &HtmlToken::Text(bar.into())));
  }

  // #[test]
  // fn event_handlers_work() {
  //   let on_click = get_event_handler();
  //   let on_click2 = get_event_handler();
  //   let dom = jsx!(<div OnClick={on_click} />);
  //   assert!(equal_enough(&dom, &HtmlToken::DomElement(DomElement {
  //     node_type: "div".into(),
  //     children: vec![],
  //     attributes: HashMap::new(),
  //     event_handlers: EventHandlers::new(),
  //   })));
  // }

  // #[test]
  // fn event_handlers_are_more_complicated() {
  //   let on_click = get_event_handler();
  //   let on_click2 = get_event_handler();
  //   let on_mouse_over = get_event_handler();
  //   let on_mouse_over2 = get_event_handler();
  //   let on_mouse_out = get_event_handler();
  //   let on_mouse_out2 = get_event_handler();
  //   let dom = jsx!(<div OnClick={on_click} OnMouseOver={on_mouse_over}>
  //     <h1 OnMouseOut={on_mouse_out} />
  //   </div>);

  //   assert!(equal_enough(
  //     &dom,
  //     &HtmlToken::DomElement(DomElement {
  //       node_type: "div".into(),
  //       children: vec![
  //         HtmlToken::DomElement(DomElement {
  //           node_type: "h1".into(),
  //           children: vec![],
  //           attributes: HashMap::new(),
  //           event_handlers: EventHandlers::new(),
  //         })
  //       ],
  //       attributes: HashMap::new(),
  //       event_handlers: EventHandlers::new(),
  //     })
  //   ))
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
