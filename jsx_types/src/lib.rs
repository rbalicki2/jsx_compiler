#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate custom_derive;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;

custom_derive! {
  #[derive(Debug, EnumFromStr, Eq, PartialEq, Hash, Clone, EnumDisplay)]
  pub enum EventName {
    OnClick,
  }
}

pub struct Event {}

pub type EventHandler = FnOnce(Event) -> ();
pub type EventHandlers = HashMap<EventName, Box<EventHandler>>;

#[derive(Debug)]
pub enum HtmlToken {
  Text(String),
  DomElement(DomElement),
}

pub struct DomElement {
  pub node_type: String,
  pub children: Vec<HtmlToken>,
  pub attributes: Attributes,
  // TODO add this if there is time
  pub event_handlers: EventHandlers,
}

impl fmt::Debug for DomElement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "DomElement {{ node_type: {}, children: {:?}, attributes: {:?}, event_handlers with keys: {:?} }}",
      self.node_type,
      self.children,
      self.attributes,
      self.event_handlers.keys().map(|e| e.to_string())
    )
  }
}

pub type Attributes = HashMap<String, String>;

impl From<String> for HtmlToken {
  fn from(s: String) -> Self {
    HtmlToken::Text(s)
  }
}

impl<'a> From<&'a str> for HtmlToken {
  fn from(s: &'a str) -> Self {
    HtmlToken::Text(s.into())
  }
}
