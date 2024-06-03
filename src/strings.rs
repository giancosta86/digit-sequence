use crate::{CrateError, CrateResult, DigitSequence};
use std::{fmt::Display, str::FromStr};

/// The string representation of a [DigitSequence] is just the concatenation of its digits.
///
/// ```
/// use digit_sequence::*;
///
/// let digit_sequence: DigitSequence = DigitSequence::new();
/// assert_eq!(digit_sequence.to_string(), "");
///
/// let digit_sequence: DigitSequence = 9u8.into();
/// assert_eq!(digit_sequence.to_string(), "9");
///
/// let digit_sequence: DigitSequence = 175438u32.into();
/// assert_eq!(digit_sequence.to_string(), "175438");
/// ```
impl Display for DigitSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit)?;
        }

        Ok(())
    }
}

/// Parsing a &[str] or [String] works if it only consists of base-10 digits,
/// with the exception of the empty string:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
///
/// let sequence: DigitSequence = "".parse()?;
/// assert_eq!(sequence, []);
///
/// let sequence: DigitSequence = "2".parse()?;
/// assert_eq!(sequence, [2]);
///
/// let sequence: DigitSequence = "0302".parse()?;
/// assert_eq!(sequence, [0, 3, 0, 2]);
///
/// let sequence: DigitSequence = String::from("0302").parse()?;
/// assert_eq!(sequence, [0, 3, 0, 2]);
///
/// # Ok(())
/// # }
/// ```
///
/// Any other string pattern results in a [CrateError::NonDigitChar]:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
///
/// let result: CrateResult<DigitSequence> = "<NOT A NUMBER>".parse();
/// assert!(result == Err(CrateError::NonDigitChar('<')));
///
/// let result: CrateResult<DigitSequence> = "90xy".parse();
/// assert!(result == Err(CrateError::NonDigitChar('x')));
///
/// let result: CrateResult<DigitSequence> = "-90".parse();
/// assert!(result == Err(CrateError::NonDigitChar('-')));
///
/// let result: CrateResult<DigitSequence> = " 90".parse();
/// assert!(result == Err(CrateError::NonDigitChar(' ')));
///  
/// # Ok(())
/// # }
/// ```
impl FromStr for DigitSequence {
    type Err = CrateError;

    fn from_str(s: &str) -> CrateResult<Self> {
        let mut digits: Vec<u8> = Vec::new();

        for current_char in s.chars() {
            match current_char.to_digit(10) {
                Some(digit) => digits.push(digit as u8),
                None => return Err(CrateError::NonDigitChar(current_char)),
            }
        }

        Ok(DigitSequence(digits))
    }
}
