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
  map!(
    complete!(match_html_token),
    |token_stream| {
      quote!({
        // N.B. explicitly cast the returned value as a HtmlToken
        // because otherwise jsx!({ foo }) is valid and can be of any type.
        extern crate jsx_types;
        let mut casted: jsx_types::HtmlToken = { #token_stream.into() };
        casted.merge_string_tokens();
        casted
      })
    }
  )
);
