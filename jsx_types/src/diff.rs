use wasm_bindgen::prelude::*;

type NewInnerHtml = String;

type Path = Vec<usize>;

// #[wasm_bindgen]
#[derive(Serialize)]
pub struct ReplaceOperation {
  pub new_inner_html: NewInnerHtml,
}

// N.B. this panics... we need to enable it if we ever have
// anything more complicated than just replacing sections.
// #[wasm_bindgen]
#[derive(Serialize)]
pub enum DiffOperation {
  Replace(ReplaceOperation),
}

pub type Diff = Vec<(Path, DiffOperation)>;
