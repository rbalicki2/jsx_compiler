use std::collections::HashMap;

#[derive(Debug)]
pub enum HtmlToken {
  Text(String),
  DomElement(DomElement),
}

#[derive(Debug)]
pub struct DomElement {
  pub node_type: String,
  pub children: Vec<HtmlToken>,
  pub attributes: Attributes,
  // TODO add this if there is time
  // pub event_handlers: EventHandlers,
}

pub type Attributes = HashMap<String, String>;
