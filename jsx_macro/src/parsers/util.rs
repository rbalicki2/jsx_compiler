use super::types::*;
use proc_macro2::{Spacing, Delimiter, Group, Literal};
use nom;

pub type CharResult<'a> = JsxIResult<'a, char>;

pub fn match_punct(
  input: TokenTreeSlice,
  c_opt: Option<char>,
  spacing_opt: Option<Spacing>,
  excluded_chars: Vec<char>
) -> CharResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input.split_first() {
    Some((first, rest)) => {
      match first {
        TokenTree::Punct(ref punct) => {

          let wrong_char = c_opt.map(|c| punct.as_char() != c).unwrap_or(false);
          let wrong_spacing = spacing_opt.map(|spacing| punct.spacing() != spacing).unwrap_or(false);
          let contains_excluded_char = excluded_chars.contains(&punct.as_char());
          
          if wrong_char || wrong_spacing || contains_excluded_char {
            get_err()
          } else {
            Ok((rest, punct.as_char()))
          }
        },
        _ => get_err(),
      }
    },
    None => get_err(),
  }
}

pub type StringResult<'a> = JsxIResult<'a, String>;

pub fn match_ident(input: TokenTreeSlice, sym_opt: Option<String>) -> StringResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input.split_first() {
    Some((first, rest)) => {
      match first {
        TokenTree::Ident(ref ident) => {
          let get_success = || Ok((rest, format!("{}", ident)));
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
    },
    None => get_err(),
  }
}

pub type GroupResult<'a> = JsxIResult<'a, Group>;

pub fn match_group(input: TokenTreeSlice, delimiter_opt: Option<Delimiter>) -> GroupResult {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input[0] {
    TokenTree::Group(ref group) => {
      let get_success = || Ok((&input[1..], group.clone() ));
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

pub fn match_literal(input: TokenTreeSlice) -> JsxIResult<Literal> {
  let get_err = || Err(nom::Err::Error(error_position!(input, nom::ErrorKind::Custom(42))));

  match input.split_first() {
    Some((first, rest)) => {
      match first {
        TokenTree::Literal(ref literal) => Ok((
          rest,
          literal.clone(),
        )),
        _ => get_err(),
      }
    },
    None => get_err(),
  }
}
