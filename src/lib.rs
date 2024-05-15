//! This crate revolves around [DigitSequence],
//! a sequence of 0-9 [u8] digits, with:
//! * conversions from integers, numeric sequences and strings
//!
//! * different iteration strategies
//!
//! * a custom [CrateResult] and a custom [CrateError]
//!
//! * optional `serde` I/O

mod arrays;
mod digit_sequence;
mod integers;
mod result;
mod slices;
mod strings;
mod vecs;

pub use digit_sequence::*;
pub use result::*;
