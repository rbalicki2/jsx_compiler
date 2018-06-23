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
