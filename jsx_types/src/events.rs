pub type EventHandler<'a, T> = 'a + FnMut(&T) -> ();
pub use web_sys::{
  Element,
  Document,
  Window,
  KeyboardEvent,
  FocusEvent,
  MouseEvent,
  InputEvent,
  HtmlElement,
  EventTarget,
  Event,
  Node
};

pub type KeyboardEventHandler<'a> = EventHandler<'a, KeyboardEvent>;
pub type FocusEventHandler<'a> = EventHandler<'a, FocusEvent>;
pub type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;
pub type InputEventHandler<'a> = EventHandler<'a, InputEvent>;

#[derive(Default)]
pub struct EventHandlers<'a> {
  // --Clipboard
  // onCopy
  // onCut
  // onPaste
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  pub on_keydown: Option<Box<KeyboardEventHandler<'a>>>,
  pub on_keypress: Option<Box<KeyboardEventHandler<'a>>>,
  pub on_keyup: Option<Box<KeyboardEventHandler<'a>>>,
  // --Focus
  pub on_focus: Option<Box<FocusEventHandler<'a>>>,
  pub on_blur: Option<Box<FocusEventHandler<'a>>>,
  // --Form
  // onChange
  pub on_change: Option<Box<InputEventHandler<'a>>>,
  pub on_input: Option<Box<InputEventHandler<'a>>>,
  // onInvalid
  pub on_submit: Option<Box<InputEventHandler<'a>>>,
  // --Mouse
  pub on_click: Option<Box<MouseEventHandler<'a>>>,
  pub on_contextmenu: Option<Box<MouseEventHandler<'a>>>,
  pub on_dblclick: Option<Box<MouseEventHandler<'a>>>,
  // onDrag
  // onDragEnd
  // onDragEnter
  // onDragExit
  // onDragLeave
  // onDragOver
  // onDragStart
  // onDrop
  // onMouseDown
  // onMouseEnter
  // onMouseLeave
  // onMouseMove
  pub on_mouseover: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouseout: Option<Box<MouseEventHandler<'a>>>,
  // onMouseUp
  // --Pointer
  // onPointerDown
  // onPointerMove
  // onPointerUp
  // onPointerCancel
  // onGotPointerCapture
  // onLostPointerCapture
  // onPointerEnter
  // onPointerLeave
  // onPointerOver
  // onPointerOut
  // --Selection
  // onSelect
  // --Touch
  // onTouchCancel
  // onTouchEnd
  // onTouchMove
  // onTouchStart
  // --Scroll
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
  // onLoad
  // onError
  // --Animatin
  // onAnimationStart
  // onAnimationEnd
  // onAnimationIteration
  // --Transition
  // onTransitionEnd
  // --Other
  // onToggle
}

macro_rules! push_to_vec {
  ($self_expr:expr, $vec:ident, $handler:ident, $handler_name:expr) => {
    if let Some(_) = $self_expr.$handler {
      $vec.push($handler_name);
    }
  };
}

impl<'a> EventHandlers<'a> {
  pub fn present_handlers(&self) -> Vec<&'static str> {
    let mut v = vec![];
    push_to_vec!(self, v, on_focus, "on_focus");
    push_to_vec!(self, v, on_focus, "on_blur");
    v
  }
}
