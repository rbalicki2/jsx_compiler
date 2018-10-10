#![feature(nll)]

extern crate wasm_bindgen;
#[macro_use]
extern crate serde_derive;
extern crate take_mut;
extern crate web_sys;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
pub mod events;
pub mod bare;
pub mod diff;

use events::*;

pub struct WrappedVector<T>(pub Vec<T>);

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
      .map(|(key, val)|
        // TODO figure out why we're cloning here!
        val.clone().map(|v| format!("{}=\"{}\"", key, v))
          .unwrap_or_else(|| key.to_string())
      )
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

pub type Attributes = HashMap<String, Option<String>>;

impl<'a> From<HtmlToken<'a>> for WrappedVector<HtmlToken<'a>> {
  fn from(t: HtmlToken<'a>) -> Self {
    WrappedVector(vec![t])
  }
}

impl<'a, T> From<Option<T>> for HtmlToken<'a> where T: Into<HtmlToken<'a>> {
  fn from(opt: Option<T>) -> Self {
    match opt {
      Some(t) => t.into(),
      None => HtmlToken::Text("".to_string()),
    }
  }
}

impl<'a, T> From<Option<T>> for WrappedVector<HtmlToken<'a>> where T: Into<HtmlToken<'a>> {
  fn from(opt: Option<T>) -> Self {
    WrappedVector(opt.into_iter().map(|t| t.into()).collect())
  }
}

impl<'a> From<String> for HtmlToken<'a> {
  fn from(s: String) -> Self {
    HtmlToken::Text(s)
  }
}

impl<'a> From<String> for WrappedVector<HtmlToken<'a>> {
  fn from(s: String) -> Self {
    WrappedVector(vec![s.into()])
  }
}

impl<'a, 'b> From<&'b str> for HtmlToken<'a> {
  fn from(s: &str) -> Self {
    HtmlToken::Text(s.to_string())
  }
}

impl<'a, 'b> From<&'b str> for WrappedVector<HtmlToken<'a>> {
  fn from(s: &str) -> Self {
    WrappedVector(vec![s.into()])
  }
}

impl<'a> From<i32> for HtmlToken<'a> {
  fn from(i: i32) -> Self {
    HtmlToken::Text(i.to_string())
  }
}

impl<'a> From<i32> for WrappedVector<HtmlToken<'a>> {
  fn from(i: i32) -> Self {
    WrappedVector(vec![i.into()])
  }
}

impl<'a> From<u32> for HtmlToken<'a> {
  fn from(u: u32) -> Self {
    HtmlToken::Text(u.to_string())
  }
}

impl<'a> From<u32> for WrappedVector<HtmlToken<'a>> {
  fn from(u: u32) -> Self {
    WrappedVector(vec![u.into()])
  }
}

impl<'a> From<usize> for HtmlToken<'a> {
  fn from(u: usize) -> Self {
    HtmlToken::Text(u.to_string())
  }
}

impl<'a> From<usize> for WrappedVector<HtmlToken<'a>> {
  fn from(u: usize) -> Self {
    WrappedVector(vec![u.into()])
  }
}

impl<'a> From<char> for HtmlToken<'a> {
  fn from(c: char) -> Self {
    HtmlToken::Text(c.to_string())
  }
}

impl<'a> From<char> for WrappedVector<HtmlToken<'a>> {
  fn from(c: char) -> Self {
    WrappedVector(vec![c.into()])
  }
}

impl<'a> From<Vec<HtmlToken<'a>>> for WrappedVector<HtmlToken<'a>> {
  fn from(v: Vec<HtmlToken<'a>>) -> Self {
    WrappedVector(v)
  }
}

// TODO make a macro or implement more

// TODO make a TopLevelComponent trait/type alias
pub trait Component<'a, T> {
  fn render(&'a mut self, T) -> HtmlToken<'a>;
}

pub trait StatelessComponent<'a, T> {
  // N.B. maybe we should rename render to render_stateless
  fn render(T) -> HtmlToken<'a>;
}

pub trait AsInnerHtml {
  fn as_inner_html(&self) -> String; 
}