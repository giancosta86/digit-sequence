use std::fmt::Display;

/// Custom version of [Result](std::result::Result), based on this crate's [Error].
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error scenarios related to this [crate].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// When a character does not represent a 0-9 digit.
    NonDigitChar(char),

    /// When a number does not represent a 0-9 digit.
    NonDigitNumber(u128),

    /// When trying conversion from a negative number.
    NegativeNumber(i128),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NonDigitChar(non_digit_char) => write!(f, "Non-digit char: {}", non_digit_char),

            Error::NonDigitNumber(non_digit_number) => {
                write!(f, "Non-digit number: {}", non_digit_number)
            }

            Error::NegativeNumber(number) => {
                write!(f, "Cannot convert negative number: {}", number)
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Converting Error to string" {
            describe "when a number is not a digit" {
                it "should describe it" {
                    eq!(Error::NonDigitNumber(90).to_string(), "Non-digit number: 90");
                }
            }

            describe "when a char is not a digit" {
                it "should describe it" {
                    eq!(Error::NonDigitChar('X').to_string(), "Non-digit char: X");
                }
            }

            describe "when trying from a negative number" {
                it "should describe it" {
                    eq!(Error::NegativeNumber(-90).to_string(), "Cannot convert negative number: -90");
                }
            }
        }
    }
}
