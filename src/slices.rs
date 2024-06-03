use crate::{CrateError, CrateResult, DigitSequence};

/// A slice of [u8] numbers can be converted to [DigitSequence]
/// as long as its values are 0-9 digits; the empty slice is
/// also acceptable.
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
///
/// let slice: &[u8] = &[];
/// let sequence: DigitSequence = slice.try_into()?;
/// assert_eq!(sequence, []);
///
/// let slice: &[u8] = &[9];
/// let sequence: DigitSequence = slice.try_into()?;
/// assert_eq!(sequence, [9]);
///
/// let slice: &[u8] = &[0, 5, 6, 9, 0];
/// let sequence: DigitSequence = slice.try_into()?;
/// assert_eq!(sequence, [0, 5, 6, 9, 0]);
///
/// # Ok(())
/// # }
/// ```
///
/// Out-of-range numbers result in a [CrateError::NonDigitNumber]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let slice: &[u8] = &[10];
/// let sequence_result: CrateResult<DigitSequence> = slice.try_into();
///
/// assert_eq!(sequence_result, Err(CrateError::NonDigitNumber(10)));
///
/// # Ok(())
/// # }
/// ```
impl TryFrom<&[u8]> for DigitSequence {
    type Error = CrateError;

    fn try_from(digits: &[u8]) -> CrateResult<Self> {
        let mut digits_vec = vec![];

        for &digit in digits {
            if digit >= 10 {
                return Err(CrateError::NonDigitNumber(digit as u128));
            }

            digits_vec.push(digit);
        }

        Ok(DigitSequence(digits_vec))
    }
}

/// [DigitSequence] can be compared with a slice of [u8]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let source: [u8; 4] = [8, 2, 5, 7];
/// let slice: &[u8] = &source;
///
/// let sequence: DigitSequence = slice.try_into()?;
/// assert_eq!(sequence, slice);
///
/// let other: [u8; 4] = [8, 9, 0, 0];
/// let other_slice: &[u8] = &other;
///
/// assert_ne!(sequence, other_slice);
///
/// # Ok(())
/// # }
/// ```
impl PartialEq<&[u8]> for DigitSequence {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}
