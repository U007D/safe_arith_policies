#![feature(once_cell)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::enum_glob_use,
    clippy::equatable_if_let,
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::match_like_matches_macro,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::wildcard_imports
)]
// To use the `unsafe` keyword, do not remove the `unsafe_code` attribute entirely.
// Instead, change it to `#![allow(unsafe_code)]` or preferably `#![deny(unsafe_code)]` + opt-in
// with local `#[allow(unsafe_code)]`'s on a case-by-case basis, if practical.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![allow(clippy::blanket_clippy_restriction_lints)]
// #![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
// #![allow(clippy::implicit_return)]

mod consts;
mod error;
mod policy;

pub use safe_arith_traits::{ICheckedOps, IMinMax, IOverflowingOps, ISaturatingOps, IWrappingOps};

pub use error::{io, Error, Result};
pub use policy::{OverflowingPolicy, WrappingPolicy};
