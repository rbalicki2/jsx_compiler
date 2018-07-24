custom_derive! {
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

pub struct OnClickEvent {
  shift_key: bool,
}

pub type OnClickEventHandler<'a> = 'a + FnMut(OnClickEvent) -> ();

pub struct EventHandlers<'a> {
  on_click: Option<Box<OnClickEventHandler<'a>>>,
}

impl<'a> EventHandlers<'a> {
  pub fn new() -> EventHandlers<'a> {
    EventHandlers {
      on_click: None,
    }
  }
}
