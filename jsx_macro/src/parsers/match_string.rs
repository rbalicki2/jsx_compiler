use super::types::*;
use proc_macro2::Literal;
use super::util::{match_punct, match_ident, match_literal_as_string};

named!(
  pub match_string <TokenTreeSlice, TokenStream>,
  map!(
    many_1_custom!(
      alt!(
        apply!(match_ident, None, true)
          | map!(
            apply!(match_punct, None, None, vec!['<']),
            |c| c.to_string()
          )
          | call!(match_literal_as_string)
      )
    ),
    |vec_of_strings| {
      let concatenated_str: String = vec_of_strings.join("");
      let lit = Literal::string(&concatenated_str);
      quote!{{
        extern crate jsx_types;
        jsx_types::HtmlToken::Text(#lit.into())
      }}
    }
  )
);
