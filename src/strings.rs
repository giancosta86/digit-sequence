use crate::{CrateError, CrateResult, DigitSequence};
use std::{fmt::Display, str::FromStr};

/// The string representation of a [DigitSequence] is just the concatenation of its digits.
///
/// ```
/// use digit_sequence::*;
///
/// let mut digit_sequence: DigitSequence;
///
/// digit_sequence = DigitSequence::new();
/// assert_eq!(digit_sequence.to_string(), "");
///
/// digit_sequence = 9u8.into();
/// assert_eq!(digit_sequence.to_string(), "9");
///
/// digit_sequence = 175438u32.into();
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
/// let mut sequence: DigitSequence;
///
/// sequence = "".parse()?;
/// assert_eq!(sequence, []);
///
/// sequence = "2".parse()?;
/// assert_eq!(sequence, [2]);
///
/// sequence = "0302".parse()?;
/// assert_eq!(sequence, [0, 3, 0, 2]);
///
/// sequence = String::from("0302").parse()?;
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
/// let mut result: CrateResult<DigitSequence>;
///
/// result = "<NOT A NUMBER>".parse();
/// assert!(result == Err(CrateError::NonDigitChar('<')));
///
/// result = "90xy".parse();
/// assert!(result == Err(CrateError::NonDigitChar('x')));
///
/// result = "-90".parse();
/// assert!(result == Err(CrateError::NonDigitChar('-')));
///
/// result = " 90".parse();
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
