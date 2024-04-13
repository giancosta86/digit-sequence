use crate::{digit_sequence::DigitSequence, CrateError, CrateResult};

macro_rules! impl_try_to_unsigned {
    ($type: ty) => {
        impl TryFrom<DigitSequence> for $type {
            type Error = CrateError;

            fn try_from(value: DigitSequence) -> CrateResult<Self> {
                Self::try_from(&value)
            }
        }

        impl TryFrom<&DigitSequence> for $type {
            type Error = CrateError;

            fn try_from(value: &DigitSequence) -> CrateResult<Self> {
                let mut result = 0 as Self;

                let enumerated_increasing_digits = value.iter().rev().enumerate();

                for (index, &digit) in enumerated_increasing_digits {
                    let power_of_ten: u32 = index.try_into().or(Err(CrateError::Overflow))?;

                    let magnitude = (10 as Self)
                        .checked_pow(power_of_ten)
                        .ok_or(CrateError::Overflow)?;

                    let addition_term = (digit as Self)
                        .checked_mul(magnitude)
                        .ok_or(CrateError::Overflow)?;

                    result = result
                        .checked_add(addition_term)
                        .ok_or(CrateError::Overflow)?;
                }

                Ok(result)
            }
        }
    };
}

impl_try_to_unsigned!(u128);
impl_try_to_unsigned!(u64);
impl_try_to_unsigned!(u32);
impl_try_to_unsigned!(u16);
impl_try_to_unsigned!(u8);
impl_try_to_unsigned!(usize);

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Converting a digit sequence to an integer" {
            fn test_case_ok<E, T>(source: T)
            where
                E: Debug,
                T: Into<DigitSequence>
                    + TryFrom<DigitSequence, Error = E>
                    + PartialEq<T>
                    + Debug
                    + Copy,
            {
                let sequence: DigitSequence = source.into();
                let roundtrip: T = sequence.try_into().unwrap();

                eq!(roundtrip, source);
            }

            fn test_case_failure(source: &str) {
                let sequence: DigitSequence = source.parse().unwrap();
                let conversion_result: CrateResult<u128> = sequence.try_into();

                eq!(conversion_result.unwrap_err(), CrateError::Overflow);
            }

            it "should convert to u8" {
                test_case_ok(90u8);
            }

            it "should convert to u16" {
                test_case_ok(90u16);
            }

            it "should convert to u32" {
                test_case_ok(90u32);
            }

            it "should convert to u64" {
                test_case_ok(90u64);
            }

            it "should convert to u128" {
                test_case_ok(90u128);
            }

            it "should convert to usize" {
                test_case_ok(90usize);
            }

            it "should convert 0" {
                test_case_ok(0u8);
            }

            it "should convert u128::MAX" {
                test_case_ok(u128::MAX);
            }

            it "should NOT convert u128::MAX + 1" {
                test_case_failure("340282366920938463463374607431768211456");
            }

            it "should NOT convert a huge sequence of 1" {
                test_case_failure("1".repeat(100).as_str());
            }
        }

        describe "Converting a reference to digit sequence to an integer" {
            it "should work" {
                let source = 90u8;

                let sequence: DigitSequence = source.into();

                let roundtrip: u8 = (&sequence).try_into().unwrap();

                eq!(roundtrip, source);

            }

            macro_rules! test_case_ok(
                ($source: expr, $type: ty) => {
                    let sequence: DigitSequence = $source.into();
                    let reference = &sequence;
                    let roundtrip: $type = reference.try_into().unwrap();

                    eq!(roundtrip, $source);
                }
            );

            fn test_case_failure(source: &str) {
                let sequence: DigitSequence = source.parse().unwrap();
                let reference = &sequence;
                let conversion_result: CrateResult<u128> = reference.try_into();

                eq!(conversion_result.unwrap_err(), CrateError::Overflow);
            }

            it "should convert to u8" {
                test_case_ok!(90u8, u8);
            }

            it "should convert to u16" {
                test_case_ok!(90u16, u16);
            }

            it "should convert to u32" {
                test_case_ok!(90u32, u32);
            }

            it "should convert to u64" {
                test_case_ok!(90u64, u64);
            }

            it "should convert to u128" {
                test_case_ok!(90u128, u128);
            }

            it "should convert to usize" {
                test_case_ok!(90usize, usize);
            }

            it "should convert 0" {
                test_case_ok!(0u8, u8);
            }

            it "should convert u128::MAX" {
                test_case_ok!(u128::MAX, u128);
            }

            it "should NOT convert u128::MAX + 1" {
                test_case_failure("340282366920938463463374607431768211456");
            }

            it "should NOT convert a huge sequence of 1" {
                test_case_failure("1".repeat(100).as_str());
            }

            it "should NOT convert a 1 of huge magnitude" {
                test_case_failure(format!("1{}", "0".repeat(100)).as_str());
            }
        }

    }
}
