#[macro_use]
mod many_0_custom;
mod types;
mod match_group;
mod fail_at_parsing;
mod match_dom_element;
mod util;

use proc_macro2::TokenStream;

use self::types::*;
use self::match_group::*;
use self::fail_at_parsing::*;
use self::match_dom_element::*;

named!(
  match_string <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_dom_element
      | match_string
      | match_bracketed_group_to_tokens
  )
);
