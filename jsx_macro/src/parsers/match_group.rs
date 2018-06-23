use super::types::*;
use proc_macro::{Group, Delimiter};
use nom;

type GroupResult<'a> = JsxIResult<'a, Group>;

pub fn match_group_fn(input: TokenTreeSlice) -> GroupResult {
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

// enable these when needed, since we can't #[allow(dead_code)] on them :(
// named!(
//   pub match_group <TokenTreeSlice, Group>,
//   call!(match_group_fn)
// );

// named!(
//   pub match_group_to_tokens <TokenTreeSlice, TokenStream>,
//   map!(
//     match_group,
//     |group| group.stream()
//   )
// );

pub fn match_group_by_delimiter_fn(input: TokenTreeSlice, delimiter: Delimiter) -> GroupResult {
  let result = match_group_fn(input);

  result.and_then(|jsx_i_result| {
    if jsx_i_result.1.delimiter() == delimiter {
      Ok(jsx_i_result)
    } else {
      Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))))
    }
  })
}

named!(
  pub match_bracketed_group_to_tokens <TokenTreeSlice, TokenStream>,
  map!(
    apply!(match_group_by_delimiter_fn, Delimiter::Brace),
    |group| group.stream()
  )
);
