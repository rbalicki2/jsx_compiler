use super::types::*;
use super::fail_at_parsing::*;
use proc_macro::{Spacing, Delimiter};
use super::util::{match_punct, match_ident, match_group, match_literal};

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
  match_attribute <TokenTreeSlice, (String, LiteralOrGroup)>,
  map!(
    tuple!(
      apply!(match_ident, None),
      apply!(match_punct, '=', None),
      alt!(
        map!(
          apply!(match_group, Some(Delimiter::Brace)),
          |group| group.into()
        )
          | map!(
            call!(match_literal),
            |literal| literal.into()
          )
      )
    ),
    // drop the equal sign
    |x| (x.0, x.2)
  )
);

named!(
  match_self_closing_tag <TokenTreeSlice, TokenStream>,
  map!(
    delimited!(
      apply!(match_punct, '<', Some(Spacing::Alone)),
      tuple!(
        apply!(match_ident, None),
        match_attribute
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
