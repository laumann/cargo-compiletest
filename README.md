cargo-compiletest
=================

This crate provides a way to use the `compiletest` utility as a Cargo
subcommand.

Installation
------------
```
cargo build --release
export PATH=$PATH:`pwd`/target/release
```

Go to your project that uses the `[compiletest_rs][compiletest]` testing harness and type:
```
cargo compiletest
```
to run your compiletest suite.

How to set up tests
-------------------
For instructions on how to set up and use compiletest, see
[laumann/compiletest-rs][compiletest].


[compiletest]: https://github.com/laumann/compiletest-rs
