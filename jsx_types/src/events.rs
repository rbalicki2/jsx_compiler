use std::default::Default;

pub type EventHandler<'a, T> = 'a + FnMut(&T) -> ();
pub use web_sys::{
  Element,
  Document,
  Window,
  MouseEvent,
  KeyboardEvent,
  InputEvent,
  HtmlElement,
  EventTarget,
  Event,
  Node
};

pub type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;

pub type KeyboardEventHandler<'a> = EventHandler<'a, KeyboardEvent>;

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
  // onFocus
  // onBlur
  // --Form
  // onChange
  pub on_input: Option<Box<InputEventHandler<'a>>>,
  // onInvalid
  // onSubmit
  // --Mouse
  pub on_click: Option<Box<MouseEventHandler<'a>>>,
  // onContextMenu
  // onDoubleClick
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
