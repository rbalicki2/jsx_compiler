#![feature(fnbox)]

#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
pub mod events;
pub mod bare;

use events::*;

#[derive(Debug)]
pub enum HtmlToken<'a> {
  Text(String),
  DomElement(DomElement<'a>),
}

impl<'a> HtmlToken<'a> {
  pub fn make_bare_dom_element(&self) -> bare::BareHtmlToken {
    match self {
      HtmlToken::Text(t) => bare::BareHtmlToken::Text(t.clone()),
      HtmlToken::DomElement(d) => bare::BareHtmlToken::DomElement(d.make_bare_dom_element()),
    }
  }
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
  pub fn make_bare_dom_element(&self) -> bare::BareDomElement {
    bare::BareDomElement {
      node_type: self.node_type.clone(),
      children: self.children.iter().map(|c| c.make_bare_dom_element()).collect::<Vec<bare::BareHtmlToken>>(),
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
      "DomElement {{ node_type: {}, children: {:?}, attributes: {:?}, event_handlers with keys: <not impl> }}",
      self.node_type,
      self.children,
      self.attributes,
      // self.event_handlers.keys().map(|e| e.to_string()).collect::<Vec<String>>()
    )
  }
}

pub type Attributes = HashMap<String, String>;

impl<'a, T> From<T> for HtmlToken<'a> where T: ToString {
  fn from(t: T) -> Self {
    HtmlToken::Text(t.to_string())
  }
}

pub trait Component<'a> {
  fn render(&'a mut self) -> HtmlToken<'a>;
}


pub trait AsInnerHtml {
  fn as_inner_html(&self) -> String; 
}