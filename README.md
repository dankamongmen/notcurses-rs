# notcurses

[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)

A higher level Rust wrapper for the [notcurses C library][1].

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


## differences with the notcurses C API

This crate depends on libnotcurses-sys which wraps the C API very closely, but
it itself offers a different approach to its public API.

If you come from libnotcurses-sys, or the C API, these are the main differences:

- You use the builder pattern to construct `Plane` and `Visual` objects.
- The concept of the standard plane disappears, so you just use simple `Plane`s,
  which internally are part of the same, or several, piles.
- Types have implemented the `Drop` trait so that you don't have to manually
  stop the `Notcurses` context, or destroy `Planes` or `Visual`s.
- The coordinate pairs (`X`, `Y`) and (`cols`, `rows`) are shown in the commont
  alphabetic order, be it as part of a function name or as function parameters.
- The `Rgb` type has many From/Into implementations, that makes it very easy to use.
- The new unified *error* and *result* types are called `Error` and `Result`.
- The types used for flags, like `Style`, are created with the `bitflags!` macro.

