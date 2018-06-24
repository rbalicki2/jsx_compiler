use super::types::*;
use proc_macro::{Spacing, Delimiter, Group};
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

pub fn match_ident(input: TokenTreeSlice, sym_opt: Option<String>) -> StringResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input[0] {
    TokenTree::Ident(ref ident) => {
      let get_success = || Ok(( &input[1..], format!("{}", ident)));
      match sym_opt {
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

pub type GroupResult<'a> = JsxIResult<'a, Group>;

pub fn match_group(input: TokenTreeSlice, delimiter_opt: Option<Delimiter>) -> GroupResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input[0] {
    TokenTree::Group(ref group) => {
      let get_success = || Ok(( &input[1..], group.clone() ));
      match delimiter_opt {
        Some(delimiter) => {
          if group.delimiter() == delimiter {
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
