type NewInnerHtml = String;

pub type Path = Vec<usize>;

// #[wasm_bindgen]
#[derive(Serialize, Debug)]
pub struct ReplaceOperation {
  pub new_inner_html: NewInnerHtml,
}

#[derive(Serialize, Debug)]
pub struct InsertOperation {
  pub new_inner_html: NewInnerHtml,
}

#[derive(Serialize, Debug)]
pub struct DeleteOperation {}

// N.B. this panics... we need to enable it if we ever have
// anything more complicated than just replacing sections.
// #[wasm_bindgen]
#[derive(Serialize, Debug)]
pub enum DiffOperation {
  Replace(ReplaceOperation),
  Insert(InsertOperation),
  Delete(DeleteOperation),
}

impl DiffOperation {
  pub fn initial_diff(inner_html: &str) -> Diff {
    vec![
      (
        vec![],
        DiffOperation::Replace(
          ReplaceOperation {
            new_inner_html: inner_html.to_string(),
          }
        )
      )
    ]
  }
}

pub type DiffItem = (Path, DiffOperation);
pub type Diff = Vec<DiffItem>;
