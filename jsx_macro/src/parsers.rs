use nom;
use nom::IResult;

use proc_macro::{TokenTree, TokenStream};
// use quote::ToTokens;

type TokenTreeSlice<'a> = &'a [TokenTree];
type JsxIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;

fn fail_at_parsing(input: TokenTreeSlice) -> JsxIResult<TokenStream> {
  Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))))
}

named!(
  match_dom_element <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);

named!(
  match_string <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);

named!(
  match_group <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_dom_element
      | match_string
      | match_group
  )
);
