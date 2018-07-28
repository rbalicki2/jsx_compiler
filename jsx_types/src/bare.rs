use super::{Attributes, AsInnerHtml};
use super::diff::{
  Diff,
  DiffItem,
  DiffOperation,
  Path,
  ReplaceOperation,
  InsertOperation,
  DeleteOperation,
  UpdateAttributesOperation,
};

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
  /**
   * Diffing algorithm
   *
   * - If it's a string, compare strings
   * - If the node_type and attributes are the same, keep it, and:
   *   - for each existing child
   *     - If it has the same node_type
   *       - keep it, and repeat
   * - If it has a different node_type or attributes, add it
   * - for each additional new child, add it
   *
   * - Thus <div><h1><h2 /></h1></div> to <div><h1><h3 /></h1></div>
   *   should see that div is the same, see that h1 is the same,
   *   see that "h2" !== "h3", and create a diff operation for
   *   that.
   *
   * - In all cases, "other" is the old rendered stuff that we
   *   want to get rid of efficiently.
   */
  pub fn get_diff_with(&self, other: &Self) -> Diff {
    self.get_path_diff_with(other, vec![0])
  }

  fn get_path_diff_with(&self, other: &Self, path: Path) -> Diff {
    match (self, other) {
      (BareHtmlToken::Text(self_text), BareHtmlToken::Text(other_text)) => {
        BareHtmlToken::get_diff_from_strings(self_text, other_text, path)
          .into_iter()
          .collect::<Diff>()
      },
      (BareHtmlToken::DomElement(self_dom), BareHtmlToken::DomElement(other_dom)) => {
        BareHtmlToken::get_diff_from_dom_elements(self_dom, other_dom, path)
      },
      _ => {
        vec![self.get_replace_self_diff_item(path)]
      },
    }
  }

  fn get_diff_from_strings(self_str: &str, other_str: &str, path: Path) -> Option<DiffItem> {
    if self_str == other_str {
      None
    } else {
      Some(get_replace_diff_item(self_str.to_string(), path))
    }
  }

  fn get_diff_from_dom_elements(
    self_dom: &BareDomElement,
    other_dom: &BareDomElement,
    path: Path
  ) -> Diff {
    if self_dom.node_type != other_dom.node_type {
      vec![get_replace_diff_item(self_dom.as_inner_html(), path)]
    } else if self_dom.attributes != other_dom.attributes {
      vec![
        get_update_attributes_diff_item(self_dom.attributes.clone(), path)
      ]
    } else {
      let self_children = &self_dom.children;
      let other_children = &other_dom.children;
      self_children.iter()
        .zip(0..(self_children.len()))
        .flat_map(|(&ref self_html_token, i)| {
          let mut new_path = path.clone();
          new_path.push(i);

          match other_children.get(i) {
            Some(other_html_token) => {
              self_html_token.get_path_diff_with(other_html_token, new_path)
            },
            None => {
              vec![self_html_token.get_insert_self_diff_item(new_path)]
            }
          }
        })
        .chain(
          (self_children.len()..other_children.len())
            .map(|i| {
              let mut new_path = path.clone();
              new_path.push(i);
              get_delete_diff_item(new_path)
            })
        )
        .collect::<Diff>()
    }
  }

  // N.B. a little weird that this is defined here, and not also on BareDomElement
  fn get_replace_self_diff_item(&self, path: Path) -> DiffItem {
    get_replace_diff_item(self.as_inner_html(), path)
  }

  fn get_insert_self_diff_item(&self, path: Path) -> DiffItem {
    get_insert_diff_item(self.as_inner_html(), path)
  }

}

fn get_replace_diff_item(new_inner_html: String, path: Path) -> DiffItem {
  (
    path,
    DiffOperation::Replace(
      ReplaceOperation {
        new_inner_html,
      }
    )
  )
}

fn get_insert_diff_item(new_inner_html: String, path: Path) -> DiffItem {
  (
    path,
    DiffOperation::Insert(
      InsertOperation {
        new_inner_html,
      }
    )
  )
}

fn get_delete_diff_item(path: Path) -> DiffItem {
  (
    path,
    DiffOperation::Delete(DeleteOperation {})
  )
}

fn get_update_attributes_diff_item(new_attributes: Attributes, path: Path) -> DiffItem {
  (
    path,
    DiffOperation::UpdateAttributes(UpdateAttributesOperation {
      new_attributes,
    })
  )
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
      "DomElement {{ node_type: {}, children: {:?} }}",
      self.node_type,
      self.children
    )
  }
}
