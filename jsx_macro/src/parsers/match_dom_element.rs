use proc_macro2::{Spacing, Delimiter, TokenStream};
use super::types::*;
use super::util::{match_punct, match_ident, match_group, match_literal};
use super::match_string::match_string;
use super::match_group::match_bracketed_group_to_tokens;

type AttributeOrEventHandler = (String, LiteralOrGroup);

macro_rules! match_event {
  ($key:ident, $val:ident, $attr_opt:ident, $event_opt:ident, $handler_name:ident, $handler_name_string:expr) => {
    if $key == $handler_name_string {
      return ($attr_opt, Some(quote!{
        #$event_opt
        event_handlers.$handler_name = Some(#$val);
      }));
    }
  };
}

fn generate_dom_element_tokens(
  node_type: String,
  attributes_or_event_handlers: Vec<AttributeOrEventHandler>,
  children: Vec<TokenStream>
) -> TokenStream {
  let (attribute_assignment, event_handler_assignment) = attributes_or_event_handlers
    .into_iter()
    .fold(
      (None, None),
      |(attr_opt, event_opt): (Option<TokenStream>, Option<TokenStream>), (key, val)| {
        // --Clipboard
        match_event!(key, val, attr_opt, event_opt, on_copy, "on_copy");
        match_event!(key, val, attr_opt, event_opt, on_cut, "on_cut");
        match_event!(key, val, attr_opt, event_opt, on_paste, "on_paste");
        // onCopy
        // onCut
        // onPaste
        // --Composition
        // onCompositionEnd
        // onCompositionStart
        // onCompositionUpdate
        // --Keyboard
        match_event!(key, val, attr_opt, event_opt, on_keydown, "on_keydown");
        match_event!(key, val, attr_opt, event_opt, on_keypress, "on_keypress");
        match_event!(key, val, attr_opt, event_opt, on_keyup, "on_keyup");
        // --Focus
        match_event!(key, val, attr_opt, event_opt, on_focus, "on_focus");
        match_event!(key, val, attr_opt, event_opt, on_blur, "on_blur");
        // --Form
        match_event!(key, val, attr_opt, event_opt, on_change, "on_change");
        match_event!(key, val, attr_opt, event_opt, on_input, "on_input");
        // onInvalid
        match_event!(key, val, attr_opt, event_opt, on_submit, "on_submit");
        // --Mouse
        match_event!(key, val, attr_opt, event_opt, on_click, "on_click");
        match_event!(key, val, attr_opt, event_opt, on_contextmenu, "on_contextmenu");
        match_event!(key, val, attr_opt, event_opt, on_dblclick, "on_dblclick");
        match_event!(key, val, attr_opt, event_opt, on_drag, "on_drag");
        // onDragEnd
        // onDragEnter
        // onDragExit
        // onDragLeave
        // onDragOver
        // onDragStart
        // onDrop
        match_event!(key, val, attr_opt, event_opt, on_mousedown, "on_mousedown");
        match_event!(key, val, attr_opt, event_opt, on_mouseenter, "on_mouseenter");
        match_event!(key, val, attr_opt, event_opt, on_mouseleave, "on_mouseleave");
        match_event!(key, val, attr_opt, event_opt, on_mousemove, "on_mousemove");
        match_event!(key, val, attr_opt, event_opt, on_mouseover, "on_mouseover");
        match_event!(key, val, attr_opt, event_opt, on_mouseout, "on_mouseout");
        match_event!(key, val, attr_opt, event_opt, on_mouseup, "on_mouseup");
        // --Pointer
        match_event!(key, val, attr_opt, event_opt, on_pointerdown, "on_pointerdown");
        match_event!(key, val, attr_opt, event_opt, on_pointermove, "on_pointermove");
        match_event!(key, val, attr_opt, event_opt, on_pointerup, "on_pointerup");
        // onPointerCancel
        // onGotPointerCapture
        // onLostPointerCapture
        match_event!(key, val, attr_opt, event_opt, on_pointerenter, "on_pointerenter");
        match_event!(key, val, attr_opt, event_opt, on_pointerleave, "on_pointerleave");
        match_event!(key, val, attr_opt, event_opt, on_pointerover, "on_pointerover");
        match_event!(key, val, attr_opt, event_opt, on_pointerout, "on_pointerout");
        // --Selection
        match_event!(key, val, attr_opt, event_opt, on_select, "on_select");
        // onSelect
        // --Touch
        // onTouchCancel
        // onTouchEnd
        // onTouchMove
        // onTouchStart
        // --Scroll
        match_event!(key, val, attr_opt, event_opt, on_scroll, "on_scroll");
        // onScroll
        // --Wheel
        // onWheel
        // --Media
        // onAbort
        // onCanPlay
        // onCanPlayThrough
        // onDurationChange
        // onEmptied
        // onEncrypted
        // onEnded
        // onError
        // onLoadedData
        // onLoadedMetadata
        // onLoadStart
        // onPause
        // onPlay
        // onPlaying
        // onProgress
        // onRateChange
        // onSeeked
        // onSeeking
        // onStalled
        // onSuspend
        // onTimeUpdate
        // onVolumeChange
        // onWaiting
        // --Image
        match_event!(key, val, attr_opt, event_opt, on_load, "on_load");
        match_event!(key, val, attr_opt, event_opt, on_error, "on_error");
        // --Animation
        match_event!(key, val, attr_opt, event_opt, on_animation_start, "on_animation_start");
        match_event!(key, val, attr_opt, event_opt, on_animation_end, "on_animation_end");
        match_event!(key, val, attr_opt, event_opt, on_animation_iteration, "on_animation_iteration");
        // --Transition
        // onTransitionEnd
        // --Other
        // onToggle
        match_event!(key, val, attr_opt, event_opt, on_toggle, "on_toggle");
        
        (
          Some(quote!{
            #attr_opt
            attr_map.insert(#key.into(), #val.into());
          }),
          event_opt
        )
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
        let mut event_handlers = ::jsx_types::events::EventHandlers::default();
        #token_stream
        event_handlers
      }}
    })
    .unwrap_or_else(|| quote!{ ::jsx_types::events::EventHandlers::default() });

  let children_vec = quote!{{
    let vec: Vec<::jsx_types::WrappedVector<::jsx_types::HtmlToken>> = vec![#(#children.into()),*];
    vec.into_iter().flat_map(|o| o.0).collect()
  }};

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
