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
