use super::{Attributes, DomElement, HtmlToken};

#[derive(Clone)]
pub struct BareDomElement {
  pub node_type: String,
  pub children: Vec<BareHtmlToken>,
  pub attributes: Attributes,
}

#[derive(Clone)]
pub enum BareHtmlToken {
  Text(String),
  DomElement(BareDomElement),
}
