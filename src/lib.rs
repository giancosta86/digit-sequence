//! This crate revolves around the [DigitSequence] struct,
//! a sequence of 0-9 [u8] digits, with:
//!
//! * conversions from/to integers, numeric sequences and strings
//!
//! * different iteration strategies
//!
//! * a custom [CrateResult] and a custom [CrateError]
//!
//! * optional [serde] I/O
//!
//! # Features
//!
//! This crate supports the following _optional_ features:
//!
//! - `serde`: enables JSON conversions via [serde](https://crates.io/crates/serde)

mod arrays;
mod integers;
mod iteration;
mod result;
mod slices;
mod strings;
mod vecs;

#[cfg(test)]
pub mod test_utils;

pub use result::*;

/// Immutable sequence of [u8] digits.
///
/// # Creation
///
/// A digit sequence is usually created via *conversions* -
/// both fallible and infallible - although a
/// [constructor](DigitSequence::new) is provided to instantiate
/// an empty sequence that is also its [Default] value.
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// assert_eq!(DigitSequence::new(), []);
/// assert_eq!(DigitSequence::default(), []);
///
/// let sequence: DigitSequence = [3, 8, 7].try_into()?;
/// assert_eq!(sequence, [3, 8, 7]);
///
/// assert_eq!(format!("{:?}", sequence), "DigitSequence([3, 8, 7])");
///
/// # Ok(())
/// # }
/// ```
///
/// For details and more code samples, please refer to the
/// implementations of the [From] and [TryFrom] interfaces.
///
/// # Equality
///
/// Equality is firstly supported with another [DigitSequence]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
///
/// let source = [1, 3, 5, 7, 9];
///
/// let left: DigitSequence = (&source).try_into()?;
/// let right: DigitSequence = (&source).try_into()?;
/// assert_eq!(left, right);
///
/// let other: DigitSequence = [9u8, 4u8].try_into()?;
/// assert_ne!(left, other);
///
/// # Ok(())
/// # }
/// ```
///
/// Equality is also supported for operands of other iterable types - such as
/// [[u8]], &[[u8]] or [Vec]: please, refer to the documentation for the
/// implementations of this [PartialEq].
///
///    
/// # Order
///
/// [PartialOrd] is supported for [DigitSequence] - just as expected:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let short_bigger: DigitSequence = [9].try_into()?;
/// let short_smaller: DigitSequence = [4].try_into()?;
///
/// let longest: DigitSequence = [9, 7].try_into()?;
///
/// assert!(short_bigger > short_smaller);
/// assert!(short_bigger < longest);
///
/// # Ok(())
/// # }
/// ```
///
/// # Serialization
///
/// **REQUIRES FEATURE**: `serde`.
///
/// When the `serde` feature is enabled for this crate, [DigitSequence] implements the [serde::Serialize] and [serde::Deserialize] traits.
///
/// ```
/// #[cfg(feature = "my_feature")]
/// {
///     use digit_sequence::*;
///     use serde_json::{to_string, from_str};
///
///     # fn main() -> GenericResult<()> {
///     let original: DigitSequence = 9786u16.into();
///     let expected_json = "[9,7,8,6]";
///
///     let json = to_string(&original)?;
///     assert_eq!(json, expected_json);
///
///     let deserialized: DigitSequence = from_str(expected_json)?;
///     assert_eq!(deserialized, original);
///
///     # Ok(())
///     # }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DigitSequence(pub(crate) Vec<u8>);

impl DigitSequence {
    /// Creates an empty sequence.
    ///
    /// ```
    /// use digit_sequence::DigitSequence;
    ///
    /// let sequence = DigitSequence::new();
    ///
    /// assert_eq!(sequence.iter().len(), 0);
    /// ```
    pub fn new() -> DigitSequence {
        DigitSequence(vec![])
    }

    /// Repeatable iteration over references to the digits.
    ///
    /// ```
    /// use digit_sequence::*;
    ///
    /// # fn main() -> GenericResult<()> {
    /// let source = [9, 5, 0, 2];
    /// let sequence: DigitSequence = (&source).try_into()?;
    /// let mut target: Vec<u8> = vec![];
    ///
    /// for &digit in sequence.iter() {
    ///     target.push(digit)
    /// }
    ///
    /// for &digit in sequence.iter() {
    ///     target.push(digit)
    /// }
    ///
    /// let expected_vec: Vec<u8> = [&source[..], &source[..]].concat();
    ///
    /// assert_eq!(target, expected_vec);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }

    /// Tells whether the sequence is empty.
    ///
    /// ```
    /// use digit_sequence::*;
    ///
    /// let empty = DigitSequence::new();
    /// assert!(empty.is_empty());
    ///
    /// let non_empty: DigitSequence = 84u8.into();
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
