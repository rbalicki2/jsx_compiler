use super::types::*;
use nom;

pub fn fail_at_parsing(input: TokenTreeSlice) -> JsxIResult<TokenStream> {
  Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))))
}