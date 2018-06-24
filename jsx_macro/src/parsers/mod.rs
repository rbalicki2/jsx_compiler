#[macro_use]
mod many_0_custom;
mod types;
mod match_group;
mod match_dom_element;
mod match_string;
mod util;

use proc_macro2::TokenStream;

use self::types::*;
use self::match_group::*;
use self::match_dom_element::*;
use self::match_string::*;

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_dom_element
      | match_string
      | match_bracketed_group_to_tokens
  )
);
