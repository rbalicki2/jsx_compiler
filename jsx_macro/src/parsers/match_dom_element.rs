use super::types::*;
use super::fail_at_parsing::*;
use proc_macro::{Spacing};
use super::util::{match_punct, match_ident};

fn generate_dom_element_tokens(node_type: String) -> TokenStream {
  (quote!{{
    ::jsx_types::HtmlToken::DomElement(
      ::jsx_types::DomElement {
        node_type: #node_type.into(),
        attributes: {
          ::std::collections::HashMap::new()
        },
        children: {
          vec![]
        },
      }
    )
  }}).into()
}

named!(
  match_self_closing_tag <TokenTreeSlice, TokenStream>,
  map!(
    delimited!(
      apply!(match_punct, '<', Some(Spacing::Alone)),
      tuple!(
        apply!(match_ident, None),
        // N.B. this is a stand-in for match_attributes
        // without it, tuple! returns a string :/
        opt!(call!(fail_at_parsing))
      ),
      tuple!(
        apply!(match_punct, '/', Some(Spacing::Joint)),
        apply!(match_punct, '>', None)
      )
    ),
    |s| {
      generate_dom_element_tokens(s.0)
    }
  )
);

named!(
  match_tag <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);

named!(
  pub match_dom_element <TokenTreeSlice, TokenStream>,
  alt!(
    match_self_closing_tag
      | match_tag
  )
);
