use proc_macro2::{Spacing, Delimiter, TokenStream, Ident, Span};
use super::types::*;
use super::util::{match_punct, match_ident, match_group, match_literal};
use super::match_string::match_string;
use super::match_group::match_bracketed_group_to_tokens;

type AttributeOrEventHandler = (String, LiteralOrGroup);

fn generate_dom_element_tokens(
  node_type: String,
  attributes_or_event_handlers: Vec<AttributeOrEventHandler>,
  children: Vec<TokenStream>
) -> TokenStream {
  let (attribute_assignment, event_handler_assignment) = attributes_or_event_handlers
    .into_iter()
    .fold(
      (None, None),
      |(mut attr_opt, mut event_opt): (Option<TokenStream>, Option<TokenStream>), (key, val)| {
        // TODO figure out why this is necessary
        let key_2: &str = &key;
        match key_2 {
          "OnClick" => {
            event_opt = Some(quote!{
              #event_opt
              event_handlers.on_click = Some(#val);
            });
          },
          _ => {
            attr_opt = Some(quote!{
              #attr_opt
              attr_map.insert(#key.into(), #val.into());
            });
          }
        }

        (attr_opt, event_opt)
      }
    );
  
  let attribute_assignment = attribute_assignment
    .map(|token_stream| {
      quote!{{
        let mut attr_map = ::std::collections::HashMap::new();
        #token_stream
        attr_map
      }}
    })
    .unwrap_or_else(|| quote!{ ::std::collections::HashMap::new() });

  let event_handler_assignment = event_handler_assignment
    .map(|token_stream| {
      quote!{{
        let mut event_handlers = ::jsx_types::events::EventHandlers::new();
        #token_stream
        event_handlers
      }}
    })
    .unwrap_or_else(|| quote!{ ::jsx_types::events::EventHandlers::new() });

  let children_vec = quote!{
    vec![#(#children.into()),*]
  };

  // N.B. jsx_types is in scope from mod.rs
  (quote!{{
    jsx_types::HtmlToken::DomElement(
      jsx_types::DomElement {
        node_type: #node_type.into(),
        attributes: #attribute_assignment,
        children: #children_vec,
        event_handlers: #event_handler_assignment,
      }
    )
  }}).into()
}

named!(
  match_attribute <TokenTreeSlice, AttributeOrEventHandler>,
  map!(
    tuple!(
      apply!(match_ident, None, false),
      apply!(match_punct, Some('='), None, vec![]),
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
      apply!(match_punct, Some('<'), Some(Spacing::Alone), vec![]),
      tuple!(
        apply!(match_ident, None, false),
        many_0_custom!(match_attribute)
      ),
      tuple!(
        apply!(match_punct, Some('/'), Some(Spacing::Joint), vec![]),
        apply!(match_punct, Some('>'), None, vec![])
      )
    ),
    |s| {
      generate_dom_element_tokens(s.0, s.1, vec![])
    }
  )
);

named!(
  match_opening_tag <TokenTreeSlice, (String, Vec<AttributeOrEventHandler>)>,
  delimited!(
    apply!(match_punct, Some('<'), Some(Spacing::Alone), vec![]),
    tuple!(
      apply!(match_ident, None, false),
      many_0_custom!(match_attribute)
    ),
    apply!(match_punct, Some('>'), None, vec![])
  )
);

named!(
  match_closing_tag <TokenTreeSlice, String>,
  delimited!(
    tuple!(
      apply!(match_punct, Some('<'), Some(Spacing::Joint), vec![]),
      apply!(match_punct, Some('/'), None, vec![])
    ),
    apply!(match_ident, None, false),
    apply!(match_punct, Some('>'), None, vec![])
  )
);

named!(
  match_tag <TokenTreeSlice, TokenStream>,
  map!(
    tuple!(
      match_opening_tag,
      many_0_custom!(match_html_token),
      match_closing_tag
    ),
    |s| {
      let opening_tag = s.0;
      assert_eq!(opening_tag.0, s.2);
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

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_dom_element
      | match_string
      | match_bracketed_group_to_tokens
  )
);
