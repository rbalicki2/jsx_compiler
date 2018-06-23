use super::types::*;
use proc_macro::Group;
use nom;

pub fn match_group_fn(input: TokenTreeSlice) -> JsxIResult<Group> {
  match input[0] {
    TokenTree::Group(ref group) => {
      Ok((
        &input[1..],
        group.clone()
      ))
    },
    // N.B. we can't call fail_at_parsing() because it has the wrong type, even
    // though we execute the same code...
    _ => Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))))
  }
}

named!(
  pub match_group <TokenTreeSlice, Group>,
  call!(match_group_fn)
);

named!(
  pub match_group_to_tokens <TokenTreeSlice, TokenStream>,
  map!(
    match_group,
    |group| { group.stream() }
  )
);
