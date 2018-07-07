#[macro_use]
mod many_custom;
mod types;
mod match_group;
mod match_dom_element;
mod match_string;
mod util;

use self::match_dom_element::match_html_token;
use self::types::*;

named!(
  pub match_jsx <TokenTreeSlice, TokenStream>,
  // N.B. this complete! macro does not do what I expect it to,
  // so I manually check in the calling code.
  complete!(match_html_token)
);
