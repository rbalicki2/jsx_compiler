use std::collections::HashMap;
use std::convert::From;

// N.B. will PartialEq be derivable if we have event_handlers?

#[derive(Debug, PartialEq)]
pub enum HtmlToken {
  Text(String),
  DomElement(DomElement),
}

#[derive(Debug, PartialEq)]
pub struct DomElement {
  pub node_type: String,
  pub children: Vec<HtmlToken>,
  pub attributes: Attributes,
  // TODO add this if there is time
  // pub event_handlers: EventHandlers,
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
