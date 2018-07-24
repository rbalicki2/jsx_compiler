#![feature(fnbox)]

#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate custom_derive;

extern crate wasm_bindgen;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
// use std::boxed::FnBox;
pub mod events;

use events::*;

pub struct Event {}

pub type EventHandler<'a> = 'a + FnMut(Event) -> ();
pub type EventHandlers<'a> = HashMap<EventName, Box<EventHandler<'a>>>;

#[derive(Debug)]
pub enum HtmlToken<'a> {
  Text(String),
  DomElement(DomElement<'a>),
}

pub trait AsInnerHtml {
  fn as_inner_html(&self) -> String; 
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
  pub event_handlers_2: EventHandlers2<'a>,
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
      "DomElement {{ node_type: {}, children: {:?}, attributes: {:?}, event_handlers with keys: {:?} }}",
      self.node_type,
      self.children,
      self.attributes,
      self.event_handlers.keys().map(|e| e.to_string()).collect::<Vec<String>>()
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
