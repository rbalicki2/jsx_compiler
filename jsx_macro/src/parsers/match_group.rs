use super::types::*;
use proc_macro::Delimiter;
use super::util::match_group;

named!(
  pub match_bracketed_group_to_tokens <TokenTreeSlice, TokenStream>,
  map!(
    apply!(match_group, Some(Delimiter::Brace)),
    |group| group.stream()
  )
);
