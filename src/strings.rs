use crate::digit_sequence::internal_create_digit_sequence;
use crate::{CrateError, CrateResult, DigitSequence};
use std::str::FromStr;

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

        Ok(internal_create_digit_sequence(digits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Converting &str to DigitSequence" {
            fn test_ok(source: &str, expected: &[u8]) {
                let digit_sequence: DigitSequence = source.parse().unwrap();

                eq!(digit_sequence, expected);
            }

            fn test_err(source: &str, expected_error: CrateError) {
                let result: CrateResult<DigitSequence> = source.parse();

                eq!(result, Err(expected_error));
            }

            describe "when passing an empty string" {
                it "should create an empty sequence" {
                    test_ok("", &[])
                }
            }

            describe "when passing '0'" {
                it "should work" {
                    test_ok("0", &[0]);
                }
            }

            describe "when passing '92'" {
                it "should work" {
                    test_ok("92", &[9, 2]);
                }
            }

            describe "when passing '304'" {
                it "should work" {
                    test_ok("304", &[3, 0, 4]);
                }
            }

            describe "when passing 340" {
                it "should work" {
                    test_ok("340", &[3, 4, 0]);
                }
            }

            describe "when passing 034" {
                it "should work" {
                    test_ok("034", &[0, 3, 4]);
                }
            }

            describe "when passing a negative number string" {
                it "should return Err" {
                    test_err("-89", CrateError::NonDigitChar('-'));
                }
            }

            describe "when passing a non-number string" {
                it "should return Err" {
                    test_err("<NOT A NUMBER>", CrateError::NonDigitChar('<'));
                }
            }

            describe "when passing a partially valid string" {
                it "should return Err" {
                    test_err("90xyz", CrateError::NonDigitChar('x'));
                }
            }
        }
    }
}
