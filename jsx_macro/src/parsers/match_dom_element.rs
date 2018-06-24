use super::types::*;
use proc_macro2::{Spacing, Delimiter};
use super::util::{match_punct, match_ident, match_group, match_literal};

type Attribute = (String, LiteralOrGroup);

fn generate_dom_element_tokens(
  node_type: String,
  attributes: Vec<Attribute>,
  children: Vec<TokenStream>
) -> TokenStream {
  let attribute_assignment = match attributes.len() {
    0 => quote!{ ::std::collections::HashMap::new() },
    _ => {
      let initialization = quote!{
        let mut attr_map = ::std::collections::HashMap::new();
      };
      let assignment = attributes.iter().fold(quote!{}, |accum, ref attr| {
        let key = attr.0.clone();
        let val = attr.1.clone();
        quote!{
          #accum
          attr_map.insert(#key.into(), #val.into());
        }
      });
      let returning = quote!{ attr_map };
      quote!{{
        #initialization
        #assignment
        #returning
      }}
    }
  };

  let children_vec = quote!{
    vec![#(#children),*]
  };

  (quote!{{
    extern crate jsx_types;
    jsx_types::HtmlToken::DomElement(
      jsx_types::DomElement {
        node_type: #node_type.into(),
        attributes: #attribute_assignment,
        children: #children_vec,
      }
    )
  }}).into()
}

named!(
  match_attribute <TokenTreeSlice, Attribute>,
  map!(
    tuple!(
      apply!(match_ident, None),
      apply!(match_punct, Some('='), None),
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

// TODO match_self_closing_tag and match_opening_tag are very similar.
named!(
  match_self_closing_tag <TokenTreeSlice, TokenStream>,
  map!(
    delimited!(
      apply!(match_punct, Some('<'), Some(Spacing::Alone)),
      tuple!(
        apply!(match_ident, None),
        many_0_custom!(match_attribute)
      ),
      tuple!(
        apply!(match_punct, Some('/'), Some(Spacing::Joint)),
        apply!(match_punct, Some('>'), None)
      )
    ),
    |s| {
      generate_dom_element_tokens(s.0, s.1, vec![])
    }
  )
);

named!(
  match_opening_tag <TokenTreeSlice, (String, Vec<Attribute>)>,
  delimited!(
    apply!(match_punct, Some('<'), Some(Spacing::Alone)),
    tuple!(
      apply!(match_ident, None),
      many_0_custom!(match_attribute)
    ),
    apply!(match_punct, Some('>'), None)
  )
);

named!(
  match_closing_tag <TokenTreeSlice, String>,
  delimited!(
    tuple!(
      apply!(match_punct, Some('<'), Some(Spacing::Joint)),
      apply!(match_punct, Some('/'), None)
    ),
    apply!(match_ident, None),
    apply!(match_punct, Some('>'), None)
  )
);

named!(
  match_tag <TokenTreeSlice, TokenStream>,
  map!(
    tuple!(
      match_opening_tag,
      // N.B. actually use match_html_token here!
      many_0_custom!(match_dom_element),
      match_closing_tag
    ),
    |s| {
      // TODO check opening tag and closing tag for the same tag name
      let opening_tag = s.0;
      let children = s.1;
      generate_dom_element_tokens(opening_tag.0, opening_tag.1, children)
    }
  )
);

named!(
  pub match_dom_element <TokenTreeSlice, TokenStream>,
  alt!(
    match_self_closing_tag
      | match_tag
  )
);
