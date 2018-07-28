#![feature(wasm_custom_section, wasm_import_module, proc_macro, nll)]

extern crate wasm_bindgen;
#[macro_use]
extern crate serde_derive;
extern crate take_mut;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
pub mod events;
pub mod bare;
pub mod diff;

use events::*;

#[derive(Debug)]
pub enum HtmlToken<'a> {
  Text(String),
  DomElement(DomElement<'a>),
}

impl<'a> HtmlToken<'a> {
  pub fn as_bare_token(&self) -> bare::BareHtmlToken {
    match self {
      HtmlToken::Text(t) => bare::BareHtmlToken::Text(t.clone()),
      HtmlToken::DomElement(d) => bare::BareHtmlToken::DomElement(d.as_bare_dom_element()),
    }
  }

  pub fn merge_string_tokens(&mut self) {
    match self {
      HtmlToken::Text(_) => {},
      HtmlToken::DomElement(ref mut d) => {
        for child in d.children.iter_mut() {
          child.merge_string_tokens()
        }
        take_mut::take(&mut d.children, |children| {
          consume_and_merge(children)
        })
      }
    }
  }
}

fn consume_and_merge(children: Vec<HtmlToken>) -> Vec<HtmlToken> {
  children.into_iter()
    .fold(vec![], |mut accum, child| {
      match child {
        HtmlToken::DomElement(_) => {
          accum.push(child);
          accum
        },
        HtmlToken::Text(ref current_text) => {
          let last_opt = accum.pop();
          match last_opt {
            Some(HtmlToken::Text(ref last_text)) => {
              accum.push(HtmlToken::Text(format!("{} {}", last_text, current_text)));
              accum
            },
            Some(x) => {
              accum.push(x);
              accum.push(child);
              accum
            },
            None => {
              accum.push(child);
              accum
            }
          }
        },
      }
    })
}

impl<'a> AsInnerHtml for HtmlToken<'a> {
  fn as_inner_html(&self) -> String {
    match &self {
      HtmlToken::Text(s) => s.to_string(),
      HtmlToken::DomElement(d) => d.as_inner_html(),
    }
  }
}

pub struct DomElement<'a> {
  pub node_type: String,
  pub children: Vec<HtmlToken<'a>>,
  pub attributes: Attributes,
  pub event_handlers: EventHandlers<'a>,
}

impl<'a> DomElement<'a> {
  pub fn as_bare_dom_element(&self) -> bare::BareDomElement {
    bare::BareDomElement {
      node_type: self.node_type.clone(),
      children: self.children.iter().map(|c| c.as_bare_token()).collect::<Vec<bare::BareHtmlToken>>(),
      attributes: self.attributes.clone(),
    }
  }
}

impl<'a> AsInnerHtml for DomElement<'a> {
  fn as_inner_html(&self) -> String {
    let attr_str: String = self.attributes
      .iter()
      .map(|(key, val)| format!("{}=\"{}\"", key, val))
      .collect::<Vec<String>>()
      .join(" ");

    match self.children.len() {
      0 => {
        format!("<{} {} />", self.node_type, attr_str)
      },
      _ => {
        format!(
          "<{} {}>{}</{}>",
          self.node_type,
          attr_str,
          self.children.iter().map(|c| c.as_inner_html()).collect::<Vec<String>>().join(""),
          self.node_type
        )
      }
    }
  }
}

impl<'a> fmt::Debug for DomElement<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "DomElement {{ node_type: {}, children: {:?} }}",
      self.node_type,
      self.children
    )
  }
}

pub type Attributes = HashMap<String, String>;

impl<'a, T> From<Option<T>> for HtmlToken<'a> where T: Into<HtmlToken<'a>> {
  fn from(opt: Option<T>) -> Self {
    match opt {
      Some(t) => t.into(),
      None => HtmlToken::Text("".to_string()),
    }
  }
}

impl<'a> From<String> for HtmlToken<'a> {
  fn from(s: String) -> Self {
    HtmlToken::Text(s)
  }
}

impl<'a, 'b> From<&'b str> for HtmlToken<'a> {
  fn from(s: &str) -> Self {
    HtmlToken::Text(s.to_string())
  }
}

impl<'a> From<i32> for HtmlToken<'a> {
  fn from(i: i32) -> Self {
    HtmlToken::Text(i.to_string())
  }
}

// TODO make a macro or implement more

pub trait Component<'a> {
  fn render(&'a mut self) -> HtmlToken<'a>;
}

pub trait AsInnerHtml {
  fn as_inner_html(&self) -> String; 
}