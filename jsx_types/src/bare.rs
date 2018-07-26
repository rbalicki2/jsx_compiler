use super::{Attributes, AsInnerHtml};
use super::diff::{Diff, DiffOperation};

#[derive(Clone)]
pub struct BareDomElement {
  pub node_type: String,
  pub children: Vec<BareHtmlToken>,
  pub attributes: Attributes,
}

#[derive(Clone, Debug)]
pub enum BareHtmlToken {
  Text(String),
  DomElement(BareDomElement),
}

impl BareHtmlToken {
  pub fn get_diff_with(&self, other: &BareHtmlToken) -> Diff {
    DiffOperation::initial_diff(&self.as_inner_html())
    // unimplemented!()
  }
}



impl AsInnerHtml for BareHtmlToken {
  fn as_inner_html(&self) -> String {
    match &self {
      BareHtmlToken::Text(s) => s.to_string(),
      BareHtmlToken::DomElement(d) => d.as_inner_html(),
    }
  }
}

impl AsInnerHtml for BareDomElement {
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

impl ::std::fmt::Debug for BareDomElement {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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
