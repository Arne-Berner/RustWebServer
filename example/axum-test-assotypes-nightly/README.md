### What is this?
This example will try to implement the nightly version of async traits with our current structure.

### How do i implement the nightly async trait version?
You can use the rust-toolchain toml to set the project to nightly and then use
```rust
#![feature(async_fn_in_trait)]
```
in lib.

### Why does this not work?
This example is to test the nightly async trait fast. If there is an update on the issue, this will be the fastest way, to test it. At this point (14 may 2023) it does not work, because of [this issue](https://github.com/rust-lang/rust/issues/103854).