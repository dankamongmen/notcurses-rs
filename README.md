# notcurses

[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)

A simple, higher-level Rust wrapper for the [notcurses C library][1].

## Status of the library

In v2.0.0 this library is getting reset so it can recommence from a new slate.

The previous public API has been completely eliminated, since it didn't made
sense anymore after all the big changes to the API of the [libnotcurses-sys][2]
underlying dependency over the last year.

Now that the boundaries and reach of the lower level bindings are better defined
the new API can be thought upon with better chances of being useful in bringing
it closer to the Rust ecosystem with more idiomatic higher level abstractions.

Currently the libnotcurses-sys API is being re-exported under the `sys` module.

[1]:https://github.com/dankamongmen/notcurses
[2]:https://crates.io/crates/libnotcurses-sys


## Main API differences with `libnotcurses-sys`

- Instead of using option structures, you now use the builder pattern
  to construct `Plane` and `Visual` objects.
- The concept of the standard plane disappears, you just use `Plane`s.
- Types have the `Drop` trait implemented so that you don't have to manually
  stop the `Notcurses` context, or to destroy `Plane`s or `Visual`s anymore.
- All coordinate pairs (`X`,`Y`), (`cols`,`rows`) are used in alphabetic
  order, either as part of the function name or as parameters.
- Many types have several `From` implementations in order to make it easier
  to use them in different contexts using `.into().
- `Align`, `Alpha`, `Blitter` and `Scale` are now enums. `Style` is a bitfield.
- New unified `Error` and `Result` types.

