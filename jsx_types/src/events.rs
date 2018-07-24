#[derive(Deserialize)]
pub struct OnClickEvent {
  pub shift_key: bool,
}

pub type OnClickEventHandler<'a> = 'a + FnMut(&OnClickEvent) -> ();

pub struct EventHandlers<'a> {
  pub on_click: Option<Box<OnClickEventHandler<'a>>>,
}

impl<'a> EventHandlers<'a> {
  pub fn new() -> EventHandlers<'a> {
    EventHandlers {
      on_click: None,
    }
  }
}
