# safe_arith_policies
This crate defines wrappers implementing arithmetic overflow policies in support of fully safe, 
ergonomic arithmetic in Rust.

Fully safe arithmetic is defined as arithmetic which will *fail to compile* if there is any
possibility of an overflow not handled by a policy specifically requested by the developer.

Ergonomic means a value wrapped with one of the following policies
utilized standard arithmetic operators but is still automatically covered by the specified overflow
policy behavior.

The arithmetic overflow policies are:

  - [x] `CheckedPolicy`
  - [x] `OverflowingPolicy`
  - [x] `PanickingPolicy`
  - [x] `SaturatingPolicy`
  - [x] `WrappingPolicy`

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
