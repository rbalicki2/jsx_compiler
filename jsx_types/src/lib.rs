use std::collections::HashMap;

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
