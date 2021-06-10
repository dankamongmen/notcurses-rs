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

If you come from the C API, these are the main differences:

- The concept of standard plane dissapears, and you only use Planes, which
  internally are piles of planes.
- You can use the builder pattern to construct types like `Plane`.
- The types used for flags, like `Style`, are created with the bitflags macro.


## Planes & Piles

The Piles are made of planes in a list. The first of the list is called the root.
They are rendered in order.
