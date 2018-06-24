use nom::IResult;
pub use proc_macro::{TokenTree, TokenStream, Group, Literal};

pub type TokenTreeSlice<'a> = &'a [TokenTree];
pub type JsxIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;

pub enum LiteralOrGroup {
  Literal(Literal),
  Group(Group),
}

impl From<Literal> for LiteralOrGroup {
  fn from(literal: Literal) -> Self {
    LiteralOrGroup::Literal(literal)
  }
}

impl From<Group> for LiteralOrGroup {
  fn from(group: Group) -> Self {
    LiteralOrGroup::Group(group)
  }
}
