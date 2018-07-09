#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate custom_derive;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;

custom_derive! {
  // N.B. uncomment these as they are tested and determined to work
  // N.B. it is *obvious* that this is a bad implementation, since
  // an OnClick event handler will receive a different Event than an
  // OnMouseOver, and the like.
  #[derive(Debug, EnumFromStr, Eq, PartialEq, Hash, Clone, EnumDisplay)]
  pub enum EventName {
    // -- Clipboard Events
    // OnCopy,
    // OnCut,
    // OnPaste,

    // -- Composition Events
    // OnCompositionEnd
    // OnCompositionStart
    // OnCompositionUpdate

    // -- Keyboard Events
    OnKeyDown,
    OnKeyPress,
    OnKeyUp,

    // -- Focus Events
    OnFocus,
    OnBlur,

    // -- Form Events
    OnChange,
    // OnInput,
    // OnInvalid,
    // OnSubmit,

    // -- Mouse Events
    OnClick,
    // OnContextMenu,
    // OnDoubleClick,
    // OnDrag,
    // OnDragEnd,
    // OnDragEnter,
    // OnDragExit,
    // OnDragLeave,
    // OnDragOver,
    // OnDragStart,
    // OnDrop,
    // OnMouseDown,
    // OnMouseEnter,
    // OnMouseLeave,
    // OnMouseMove,
    OnMouseOut,
    OnMouseOver,
    // OnMouseUp,

    // -- Pointer Events
    // OnPointerDown,
    // OnPointerMove,
    // OnPointerUp,
    // OnPointerCancel,
    // OnGotPointerCapture,
    // OnLostPointerCapture,
    // OnPointerEnter,
    // OnPointerLeave,
    // OnPointerOver,
    // OnPointerOut,

    // -- Selection Events
    // OnSelect,

    // -- Touch Events
    // OnTouchCancel,
    // OnTouchEnd,
    // OnTouchMove,
    // OnTouchStart,

    // -- UI Events
    // OnScroll,

    // -- Wheel Events
    // OnWheel,

    // -- Media Events
    // OnAbort,
    // OnCanPlay,
    // OnCanPlayThrough,
    // OnDurationChange,
    // OnEmptied,
    // OnEncrypted,
    // OnEnded,
    // OnError,
    // OnLoadedData,
    // OnLoadedMetadata,
    // OnLoadStart,
    // OnPause,
    // OnPlay,
    // OnPlaying,
    // OnProgress,
    // OnRateChange,
    // OnSeeked,
    // OnSeeking,
    // OnStalled,
    // OnSuspend,
    // OnTimeUpdate,
    // OnVolumeChange,
    // OnWaiting,

    // -- Image Events
    // OnLoad,
    // OnError,

    // -- Animation Events
    // OnAnimationStart,
    // OnAnimationEnd,
    // OnAnimationIteration,

    // -- Transition Events
    // OnTransitionEnd,

    // -- Other Events
    // OnToggle,
  }
}

pub struct Event {}

pub type EventHandler = FnOnce(Event) -> ();
pub type EventHandlers = HashMap<EventName, Box<EventHandler>>;

#[derive(Debug)]
pub enum HtmlToken {
  Text(String),
  DomElement(DomElement),
}

pub struct DomElement {
  pub node_type: String,
  pub children: Vec<HtmlToken>,
  pub attributes: Attributes,
  // TODO add this if there is time
  pub event_handlers: EventHandlers,
}

impl fmt::Debug for DomElement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "DomElement {{ node_type: {}, children: {:?}, attributes: {:?}, event_handlers with keys: {:?} }}",
      self.node_type,
      self.children,
      self.attributes,
      self.event_handlers.keys().map(|e| e.to_string()).collect::<Vec<String>>()
    )
  }
}

pub type Attributes = HashMap<String, String>;

impl From<String> for HtmlToken {
  fn from(s: String) -> Self {
    HtmlToken::Text(s)
  }
}

impl<'a> From<&'a str> for HtmlToken {
  fn from(s: &'a str) -> Self {
    HtmlToken::Text(s.into())
  }
}
