use super::types::*;
use proc_macro::Spacing;
use nom;

pub type CharResult<'a> = JsxIResult<'a, char>;

pub fn match_punct(input: TokenTreeSlice, c: char, spacing_opt: Option<Spacing>) -> CharResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input[0] {
    TokenTree::Punct(ref punct) => {
      let get_success = || Ok(( &input[1..], punct.as_char()));
      if punct.as_char() == c {
        match spacing_opt {
          Some(spacing) => {
            if spacing == punct.spacing() {
              get_success()
            } else {
              get_err()
            }
          },
          None => get_success()
        }
      } else {
        get_err()
      }
    },
    _ => get_err(),
  }
}

pub type StringResult<'a> = JsxIResult<'a, String>;

pub fn match_ident(input: TokenTreeSlice, opt_sym: Option<String>) -> StringResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input[0] {
    TokenTree::Ident(ref ident) => {
      let get_success = || Ok(( &input[1..], format!("{}", ident)));
      match opt_sym {
        Some(s) => {
          if s == format!("{}", ident) {
            get_success()
          } else {
            get_err()
          }
        },
        None => get_success()
      }
    },
    _ => get_err(),
  }
}
