use crate::{CrateError, CrateResult, DigitSequence};

/// [DigitSequence] can be created from a [Vec] of [u8], as long as
/// the vector contains only 0-9 digits or is empty; otherwise,
/// the conversion results in a [CrateError::NonDigitNumber]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let mut sequence: DigitSequence;
///
/// sequence = vec![].try_into()?;
/// assert_eq!(sequence, []);
///
/// sequence = vec![9, 2].try_into()?;
/// assert_eq!(sequence, [9, 2]);
///
/// let result: CrateResult<DigitSequence> = vec![10].try_into();
/// assert_eq!(result, Err(CrateError::NonDigitNumber(10)));
///
/// # Ok(())
/// # }
/// ```
impl TryFrom<Vec<u8>> for DigitSequence {
    type Error = CrateError;

    fn try_from(value: Vec<u8>) -> CrateResult<Self> {
        (&value as &[u8]).try_into()
    }
}

/// [DigitSequence] can be created from a
/// reference to a [Vec] of [u8], as long as the vector
/// only contains 0-9 digits or is empty; otherwise,
/// the conversion results in a [CrateError::NonDigitNumber]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let mut sequence: DigitSequence;
///
/// sequence = (&vec![]).try_into()?;
/// assert_eq!(sequence, []);
///
/// sequence = (&vec![9, 2]).try_into()?;
/// assert_eq!(sequence, [9, 2]);
///
/// let result: CrateResult<DigitSequence> = (&vec![10]).try_into();
/// assert_eq!(result, Err(CrateError::NonDigitNumber(10)));
///
/// # Ok(())
/// # }
/// ```
impl TryFrom<&Vec<u8>> for DigitSequence {
    type Error = CrateError;

    fn try_from(value: &Vec<u8>) -> CrateResult<Self> {
        (value as &[u8]).try_into()
    }
}

/// [DigitSequence] supports equality with a [Vec] of [u8].
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let vector = vec![1, 3, 5, 7, 9];
/// let sequence: DigitSequence = (&vector).try_into()?;
///
/// assert_eq!(sequence, vector);
///
/// let other = vec![90, 92];
/// assert_ne!(sequence, other);
///  
/// # Ok(())
/// # }
/// ```
impl PartialEq<Vec<u8>> for DigitSequence {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.0 == *other
    }
}

/// [DigitSequence] supports equality with a reference to a [Vec] of [u8].
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
///
/// let vector = vec![1, 3, 5, 7, 9];
/// let sequence: DigitSequence = (&vector).try_into()?;
///
/// assert_eq!(sequence, &vector);
///
/// let other = vec![90, 92];
/// assert_ne!(sequence, &other);
///  
/// # Ok(())
/// # }
/// ```
impl PartialEq<&Vec<u8>> for DigitSequence {
    fn eq(&self, other: &&Vec<u8>) -> bool {
        self.0 == **other
    }
}
