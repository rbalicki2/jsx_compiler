# `jsx_compiler`

> A procedural macro for compiling jsx into a `DomElement` type

## Example

```rs
#![feature(proc_macro, proc_macro_non_items)]

extern crate jsx_types;
extern crate jsx_macro;

let on_click: Box<jsx_types::EventHandler> = Box::new(|_| println!("foo!"));
let subtitle = "This is a subtitle";
let dom = jsx!(<div foo="bar">
  <h1 OnClick={on_click}>This title is clickable</h1>
  <div class="subtitle">{ subtitle }</div>
</div>);
```

## Crate Organization

### jsx_macro

* This is the main crate, which exports the jsx! and jsx_verbose! macros.

### jsx_types

* This macro exports all of the used types: `HtmlToken`, `DomElement`, `EventName`, `Event`, `EventHandler`, `Attributes` and `EventHandlers`.

### jsx_macro_tests

* Tests the `jsx_macro` crate.

## TODO

* Add RustDoc docs
* `DomElement.event_handlers` is not done correctly. For example, an `OnClick` handlers and an `OnMouseOver` handlers should receive different events. Thus, `DomElement` should have separate, optional fields, e.g.:

```
pub type DomElement {
  // ... other fields
  on_click: Option<Box<FnOnce(ClickEvent) -> ()>>,
  on_mouse_over: Option<Box<FnOnce(MouseEvent) -> ()>>,
}
```

* There should be also be a builder-pattern constructor for `DomElement`. 
* Integrate with a wasm full-stack app.
* Other methods: `to_inner_html` and the like.

## Presentation

Follow along with the [presentation](https://docs.google.com/presentation/d/11KK06J-p-Q2XLg1VW7GK02rSCn3z-pvfKf59WMxNirA/edit?usp=sharing) and with the [video](https://youtu.be/sorD8vpKHHU). The code that is current at the time of the presentation was tag v7, but you should have no trouble following along with master (as of July 8th).
