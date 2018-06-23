mod types;
mod match_group;
mod fail_at_parsing;

use proc_macro::TokenStream;

use self::types::*;
use self::match_group::*;
use self::fail_at_parsing::*;

named!(
  match_dom_element <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);

named!(
  match_string <TokenTreeSlice, TokenStream>,
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
