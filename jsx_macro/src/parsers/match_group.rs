use super::types::*;
use super::fail_at_parsing::*;

named!(
  pub match_group <TokenTreeSlice, TokenStream>,
  call!(fail_at_parsing)
);