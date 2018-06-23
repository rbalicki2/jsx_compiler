use nom::IResult;
pub use proc_macro::{TokenTree, TokenStream};

pub type TokenTreeSlice<'a> = &'a [TokenTree];
pub type JsxIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;
