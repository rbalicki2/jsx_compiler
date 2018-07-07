use super::types::*;
use proc_macro2::Literal;
use super::util::{match_punct, match_ident, match_literal};

named!(
  pub match_string <TokenTreeSlice, TokenStream>,
  map!(
    many_1_custom!(
      alt!(
        apply!(match_ident, None)
          | map!(
            apply!(match_punct, None, None, vec!['<']),
            |c| c.to_string()
          )
          | map!(
            call!(match_literal),
            |lit| lit.to_string()
          )
      )
    ),
    |vec_of_strings| {
      // N.B. need to calculate, using spans, how many extra spaces to include
      // and then to join on ""
      let concatenated_str: String = vec_of_strings.join(" ");
      let lit = Literal::string(&concatenated_str);
      quote!{{
        extern crate jsx_types;
        jsx_types::HtmlToken::Text(#lit.into())
      }}
    }
  )
);
