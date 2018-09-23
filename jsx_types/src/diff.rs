use super::Attributes;

type NewInnerHtml = String;

pub type Path = Vec<usize>;

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

#[derive(Serialize, Debug)]
pub struct UpdateAttributesOperation {
  pub new_attributes: Attributes,
}

#[derive(Serialize, Debug)]
pub enum DiffOperation {
  Replace(ReplaceOperation),
  Insert(InsertOperation),
  Delete(DeleteOperation),
  UpdateAttributes(UpdateAttributesOperation),
}

pub type DiffItem = (Path, DiffOperation);
pub type Diff = Vec<DiffItem>;
