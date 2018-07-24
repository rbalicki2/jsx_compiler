pub struct OnClickEvent {
  shift_key: bool,
}

pub type OnClickEventHandler<'a> = 'a + FnMut(OnClickEvent) -> ();

pub struct EventHandlers2<'a> {
  on_click: Option<Box<OnClickEventHandler<'a>>>,
}

impl<'a> EventHandlers2<'a> {
  pub fn new() -> EventHandlers2<'a> {
    EventHandlers2 {
      on_click: None,
    }
  }
}
