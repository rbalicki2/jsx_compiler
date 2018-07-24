pub type OnClickEventHandler<'a> = 'a + FnMut(&MouseEvent) -> ();

#[derive(Deserialize)]
pub struct MouseEvent {
  pub alt_key: bool,
  pub client_x: u32,
  pub client_y: u32,
  pub ctrl_key: bool,
  pub layer_x: u32,
  pub layer_y: u32,
  pub meta_key: bool,
  pub movement_x: u32,
  pub movement_y: u32,
  pub offset_x: u32,
  pub offset_y: u32,
  pub page_x: u32,
  pub page_y: u32,
  pub screen_x: u32,
  pub screen_y: u32,
  pub shift_key: bool,
  pub time_stamp: f32,
  // type is a reserved word
  pub event_type: String,
  pub x: u32,
  pub y: u32,
}

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
