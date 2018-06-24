#[macro_use]
mod many_0_custom;
mod types;
mod match_group;
mod match_dom_element;
mod match_string;
mod util;

pub use self::match_dom_element::match_html_token;
