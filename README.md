# `jsx_compiler`

> A procedural macro for compiling jsx into a `DomElement` type

Follow along with the [presentation](https://docs.google.com/presentation/d/11KK06J-p-Q2XLg1VW7GK02rSCn3z-pvfKf59WMxNirA/edit?usp=sharing)

## tags

* v0

Bare-bones procedural macro that does nothing to the input, and the basic types we will be using.

* v1

Create a `match_html_token` macro that uses `alt!` to match on a group, a DomElement or a string.
Hook each of these up to a an always-failing function, for now.

In the macros entrypoint, add logging of the input and final result.

* v2

Implement `match_group_to_tokens` and `match_bracketed_group_to_tokens`.

Note: because these macros don't respect `#[allow(dead_code)]`, I left some
of them commented out. But they may be useful in the future... probably not :)

(Note: we want parentheses and square brackets to have no special value inside of
an html string... but that will probably not be possible, since macros must be
balanced.)

* v3

Implement `match_self_closing_tag`, which returns tokens that become a string. It will
eventually return tokens which compile into a DomElement.

Add utility functions `match_punct` and `match_ident`. Note: the functions in
the `match_group` module should've taken the same form as these, and maybe I will refactor them.

* v4

Implement `generate_dom_element_tokens` which makes a self-closing dom element!

* v5

Implement `match_attribute` for many attributes.

Get groups working properly, i.e. with hygiene.

Upgrade to proc_macro2.

Add a `many_0_custom` macro, because == doesn't work on TokenStreams,
unlike on u8 slices.

Refactor match_group to take an Option<Delimiter>, instead of there being
two separate functions.
