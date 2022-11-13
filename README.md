# safe_arith_policies
This crate defines wrappers implementing arithmetic overflow policies in support of ergonomic, 
fully safe arithmetic in Rust.

Ergonomic means a value wrapped with one of the following policies
is automatically subject to the specified overflow behavior even when using standard arithmetic
operators.

Fully safe arithmetic is defined as arithmetic which will fail to compile if there is any
possibility of an overflow not specifically requested by the developer.

The arithmetic overflow policies are:
  * `CheckedPolicy`
  * `OverflowingPolicy`
  * `PanickingPolicy`
  * `SaturatingPolicy`
  * `WrappingPolicy`

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
