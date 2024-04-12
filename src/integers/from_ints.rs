use crate::digit_sequence::internal_create_digit_sequence;
use crate::{CrateError, CrateResult, DigitSequence};

macro_rules! impl_try_from_signed {
    ($type: ty) => {
        impl TryFrom<$type> for DigitSequence {
            type Error = CrateError;

            fn try_from(value: $type) -> CrateResult<Self> {
                if value < 0 {
                    return Err(CrateError::NegativeNumber(value as i128));
                }

                Ok(convert_from_positive!(value))
            }
        }
    };
}

macro_rules! impl_from_unsigned {
    ($type: ty) => {
        impl From<$type> for DigitSequence {
            fn from(value: $type) -> DigitSequence {
                convert_from_positive!(value)
            }
        }
    };
}

macro_rules! convert_from_positive {
    ($value: ident) => {{
        let mut reversed_digits = Vec::new();
        let mut current_value = $value;

        loop {
            let digit = current_value % 10;

            reversed_digits.push(digit as u8);

            current_value /= 10;

            if current_value == 0 {
                break;
            }
        }

        let digits: Vec<u8> = reversed_digits.into_iter().rev().collect();

        internal_create_digit_sequence(digits)
    }};
}

impl_try_from_signed!(i128);
impl_try_from_signed!(i64);
impl_try_from_signed!(i32);
impl_try_from_signed!(i16);
impl_try_from_signed!(i8);
impl_try_from_signed!(isize);

impl_from_unsigned!(u128);
impl_from_unsigned!(u64);
impl_from_unsigned!(u32);
impl_from_unsigned!(u16);
impl_from_unsigned!(u8);
impl_from_unsigned!(usize);

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Converting an integer to a digit sequence" {
            describe "when converting 0" {
                it "should return a sequence having just 0" {
                    let digit_sequence: DigitSequence = 0u8.into();

                    eq!(digit_sequence, [0]);
                }
            }

            describe "when converting a negative number" {
                it "should return Err" {
                    let result: CrateResult<DigitSequence> = (-4).try_into();

                    eq!(result, Err(CrateError::NegativeNumber(-4)));
                }
            }

            describe "when converting an unsigned" {
                fn test_case<T: Into<DigitSequence>>(source: T, expected_digits: &[u8]) {
                    let actual_sequence: DigitSequence = source.into();

                    eq!(actual_sequence, expected_digits);
                }

                it "should convert a u8" {
                    test_case(107u8, &[1, 0, 7])
                }

                it "should convert a u16" {
                    test_case(107u16, &[1, 0, 7])
                }

                it "should convert a u32" {
                    test_case(107u32, &[1, 0, 7])
                }

                it "should convert a u64" {
                    test_case(107u64, &[1, 0, 7])
                }

                it "should convert a u128" {
                    test_case(107u128, &[1, 0, 7])
                }

                it "should convert a usize" {
                    test_case(107usize, &[1, 0, 7])
                }
            }


            describe "when converting a signed" {
                fn test_case<T: TryInto<DigitSequence>>(source: T, expected_digits: &[u8])
                where <T as TryInto<DigitSequence>>::Error: Debug {
                    let actual_sequence: DigitSequence = source.try_into().unwrap();

                    eq!(actual_sequence, expected_digits);
                }

                it "should convert a i8" {
                    test_case(107i8, &[1, 0, 7])
                }

                it "should convert a i16" {
                    test_case(107i16, &[1, 0, 7])
                }

                it "should convert a i32" {
                    test_case(107i32, &[1, 0, 7])
                }

                it "should convert a i64" {
                    test_case(107i64, &[1, 0, 7])
                }

                it "should convert a i128" {
                    test_case(107i128, &[1, 0, 7])
                }

                it "should convert a isize" {
                    test_case(107isize, &[1, 0, 7])
                }
            }
        }
    }
}
