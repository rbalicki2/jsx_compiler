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
        match_event!(key, val, attr_opt, event_opt, on_key_down, "on_key_down");
        match_event!(key, val, attr_opt, event_opt, on_key_press, "on_key_press");
        match_event!(key, val, attr_opt, event_opt, on_key_up, "on_key_up");
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
        match_event!(key, val, attr_opt, event_opt, on_context_menu, "on_contextmenu");
        match_event!(key, val, attr_opt, event_opt, on_dbl_click, "on_dblclick");

        match_event!(key, val, attr_opt, event_opt, on_drag, "on_drag");
        match_event!(key, val, attr_opt, event_opt, on_drag_end, "on_drag_end");
        match_event!(key, val, attr_opt, event_opt, on_drag_enter, "on_drag_enter");
        match_event!(key, val, attr_opt, event_opt, on_drag_exit, "on_drag_exit");
        match_event!(key, val, attr_opt, event_opt, on_drag_leave, "on_drag_leave");
        match_event!(key, val, attr_opt, event_opt, on_drag_over, "on_drag_over");
        match_event!(key, val, attr_opt, event_opt, on_drag_start, "on_drag_start");
        match_event!(key, val, attr_opt, event_opt, on_drop, "on_drop");

        match_event!(key, val, attr_opt, event_opt, on_mouse_down, "on_mouse_down");
        match_event!(key, val, attr_opt, event_opt, on_mouse_enter, "on_mouse_enter");
        match_event!(key, val, attr_opt, event_opt, on_mouse_leave, "on_mouse_leave");
        match_event!(key, val, attr_opt, event_opt, on_mouse_move, "on_mouse_move");
        match_event!(key, val, attr_opt, event_opt, on_mouse_over, "on_mouse_over");
        match_event!(key, val, attr_opt, event_opt, on_mouse_out, "on_mouse_out");
        match_event!(key, val, attr_opt, event_opt, on_mouse_up, "on_mouse_up");
        // --Pointer
        match_event!(key, val, attr_opt, event_opt, on_pointer_down, "on_pointer_down");
        match_event!(key, val, attr_opt, event_opt, on_pointer_move, "on_pointer_move");
        match_event!(key, val, attr_opt, event_opt, on_pointer_up, "on_pointer_up");
        // onPointerCancel
        // onGotPointerCapture
        // onLostPointerCapture
        match_event!(key, val, attr_opt, event_opt, on_pointer_enter, "on_pointer_enter");
        match_event!(key, val, attr_opt, event_opt, on_pointer_leave, "on_pointer_leave");
        match_event!(key, val, attr_opt, event_opt, on_pointer_over, "on_pointer_over");
        match_event!(key, val, attr_opt, event_opt, on_pointer_out, "on_pointer_out");
        // --Selection
        match_event!(key, val, attr_opt, event_opt, on_select, "on_select");
        // --Touch
        match_event!(key, val, attr_opt, event_opt, on_touch_cancel, "on_touch_cancel");
        match_event!(key, val, attr_opt, event_opt, on_touch_end, "on_touch_end");
        match_event!(key, val, attr_opt, event_opt, on_touch_move, "on_touch_move");
        match_event!(key, val, attr_opt, event_opt, on_touch_start, "on_touch_start");
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
