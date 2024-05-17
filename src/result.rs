use std::error::Error;
use std::fmt::Display;

/// The most generic [Error]-based [Result].
pub type GenericResult<T> = Result<T, Box<dyn Error>>;

/// Custom version of [Result], based on this [crate]'s [CrateError].
pub type CrateResult<T> = Result<T, CrateError>;

/// Custom error scenarios related to this [crate].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CrateError {
    /// When a character does not represent a 0-9 digit.
    NonDigitChar(char),

    /// When a number does not represent a 0-9 digit.
    NonDigitNumber(u128),

    /// When trying to convert a negative number.
    NegativeNumber(i128),

    /// When an operation causes a numeric overflow.
    Overflow,
}

/// [CrateError] has a string representation.
///
/// ```
/// use digit_sequence::*;
///
/// assert_eq!(CrateError::NonDigitNumber(90).to_string(), "Non-digit number: 90");
/// assert_eq!(CrateError::NonDigitChar('X').to_string(), "Non-digit char: X");
/// assert_eq!(CrateError::NegativeNumber(-90).to_string(), "Cannot convert negative number: -90");
/// assert_eq!(CrateError::Overflow.to_string(), "Overflow");
/// ```
impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonDigitChar(non_digit_char) => write!(f, "Non-digit char: {}", non_digit_char),

            Self::NonDigitNumber(non_digit_number) => {
                write!(f, "Non-digit number: {}", non_digit_number)
            }

            Self::NegativeNumber(number) => {
                write!(f, "Cannot convert negative number: {}", number)
            }

            Self::Overflow => write!(f, "Overflow"),
        }
    }
}

impl Error for CrateError {}
