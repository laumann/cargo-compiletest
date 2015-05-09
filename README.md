cargo-compiletest
=================

This crate provides a way to use the `compiletest` utility as a Cargo
subcommand.

Installation
------------
Clone this repo and build it with `cargo build`. A binary called
`cargo-compiletest` should be produced (either in `target/debug` or
`target/release`). Ensure that it is placed somewhere on your `PATH` and you
should be good to go. Simply type

```
cargo compiletest
```

to run your compiletest suite.

How to set up tests
-------------------
For instructions on how to set up anduse compiletest, see
[laumann/compiletest-rs][1].


[1] https://github.com/laumann/compiletest-rs
