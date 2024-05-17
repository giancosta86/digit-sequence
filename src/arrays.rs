use crate::{CrateError, CrateResult, DigitSequence};

/// [DigitSequence] can be created from an array of [u8], as long as
/// the array contains only 0-9 digits or is empty; otherwise,
/// the conversion results in a [CrateError::NonDigitNumber]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let mut sequence: DigitSequence;
///
/// sequence = [].try_into()?;
/// assert_eq!(sequence, []);
///
/// sequence = [9, 2].try_into()?;
/// assert_eq!(sequence, [9, 2]);
///
/// let result: CrateResult<DigitSequence> = [10].try_into();
/// assert_eq!(result, Err(CrateError::NonDigitNumber(10)));
///
/// # Ok(())
/// # }
/// ```
impl<const N: usize> TryFrom<[u8; N]> for DigitSequence {
    type Error = CrateError;

    fn try_from(digits: [u8; N]) -> CrateResult<Self> {
        (&digits as &[u8]).try_into()
    }
}

/// [DigitSequence] can be created from a
/// reference to a [u8] array, as long as the array
/// only contains 0-9 digits or is empty; otherwise,
/// the conversion results in a [CrateError::NonDigitNumber]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let mut sequence: DigitSequence;
///
/// sequence = (&[]).try_into()?;
/// assert_eq!(sequence, []);
///
/// sequence = (&[9, 2]).try_into()?;
/// assert_eq!(sequence, [9, 2]);
///
/// let result: CrateResult<DigitSequence> = [10].try_into();
/// assert_eq!(result, Err(CrateError::NonDigitNumber(10)));
///
/// # Ok(())
/// # }
/// ```
impl<const N: usize> TryFrom<&[u8; N]> for DigitSequence {
    type Error = CrateError;

    fn try_from(digits: &[u8; N]) -> CrateResult<Self> {
        (digits as &[u8]).try_into()
    }
}

/// [DigitSequence] supports equality with a fixed-size array.
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let source = [1, 3, 5, 7, 9];
/// let sequence: DigitSequence = (&source).try_into()?;
///
/// assert_eq!(sequence, source);
///
/// let other = [9, 4];
/// assert_ne!(sequence, other);
///
/// # Ok(())
/// # }
/// ```
impl<const N: usize> PartialEq<[u8; N]> for DigitSequence {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

/// [DigitSequence] supports equality with a reference to a fixed-size array.
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let source = [1, 3, 5, 7, 9];
/// let sequence: DigitSequence = (&source).try_into()?;
///
/// assert_eq!(sequence, &source);
///
/// let other = [9, 4];
/// assert_ne!(sequence, &other);
///
/// # Ok(())
/// # }
/// ```
impl<const N: usize> PartialEq<&[u8; N]> for DigitSequence {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.0 == **other
    }
}
