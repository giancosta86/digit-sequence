use crate::{CrateError, CrateResult, DigitSequence};

macro_rules! impl_try_to_unsigned {
    ($type: ty) => {
        /// Conversion from a [DigitSequence] is only available to
        /// *unsigned* integers.
        ///
        /// It is always fallible - because
        /// it might result in a [CrateError::Overflow].
        impl TryFrom<DigitSequence> for $type {
            type Error = CrateError;

            fn try_from(sequence: DigitSequence) -> CrateResult<Self> {
                (&sequence).try_into()
            }
        }

        /// Conversion from a &[DigitSequence] is only available to
        /// *unsigned* integers.
        ///
        /// It is always fallible - because
        /// it might result in a [CrateError::Overflow].
        impl TryFrom<&DigitSequence> for $type {
            type Error = CrateError;

            fn try_from(sequence: &DigitSequence) -> CrateResult<Self> {
                let mut result = 0 as Self;

                let enumerated_increasing_digits = sequence.iter().rev().enumerate();

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
    use crate::test_utils::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Converting a digit sequence to an integer" {
            fn test_case_overflow_u128(source: &str) {
                let sequence: DigitSequence = source.parse().unwrap();
                let conversion_result: CrateResult<u128> = sequence.try_into();

                eq!(conversion_result, Err(CrateError::Overflow));
            }

            it "should convert to/from u8" {
                test_roundtrip_conversion(90u8);
            }

            it "should convert to/from u16" {
                test_roundtrip_conversion(90u16);
            }

            it "should convert to/from u32" {
                test_roundtrip_conversion(90u32);
            }

            it "should convert to/from u64" {
                test_roundtrip_conversion(90u64);
            }

            it "should convert to/from u128" {
                test_roundtrip_conversion(90u128);
            }

            it "should convert to/from usize" {
                test_roundtrip_conversion(90usize);
            }

            it "should convert 0" {
                test_roundtrip_conversion(0u8);
            }

            it "should convert u128::MAX" {
                test_roundtrip_conversion(u128::MAX);
            }

            it "should NOT convert u128::MAX + 1" {
                test_case_overflow_u128("340282366920938463463374607431768211456");
            }

            it "should NOT convert a huge sequence of 1" {
                test_case_overflow_u128("1".repeat(100).as_str());
            }

            it "should NOT convert a 1 of huge magnitude" {
                test_case_overflow_u128(&format!("1{}", "0".repeat(100)));
            }
        }

        describe "Converting a reference to digit sequence to an integer" {
            fn test_case_overflow_u128_via_ref(source: &str) {
                let sequence: DigitSequence = source.parse().unwrap();
                let reference = &sequence;
                let conversion_result: CrateResult<u128> = reference.try_into();

                eq!(conversion_result, Err(CrateError::Overflow));
            }

            it "should convert to u8" {
                test_roundtrip_conversion_via_ref(90u8);
            }

            it "should convert to u16" {
                test_roundtrip_conversion_via_ref(90u16);
            }

            it "should convert to u32" {
                test_roundtrip_conversion_via_ref(90u32);
            }

            it "should convert to u64" {
                test_roundtrip_conversion_via_ref(90u64);
            }

            it "should convert to u128" {
                test_roundtrip_conversion_via_ref(90u128);
            }

            it "should convert to usize" {
                test_roundtrip_conversion_via_ref(90usize);
            }

            it "should convert 0" {
                test_roundtrip_conversion_via_ref(0u8);
            }

            it "should convert u128::MAX" {
                test_roundtrip_conversion_via_ref(u128::MAX);
            }

            it "should NOT convert u128::MAX + 1" {
                test_case_overflow_u128_via_ref("340282366920938463463374607431768211456");
            }

            it "should NOT convert a huge sequence of 1" {
                test_case_overflow_u128_via_ref(&"1".repeat(100));
            }

            it "should NOT convert a 1 of huge magnitude" {
                test_case_overflow_u128_via_ref(&format!("1{}", "0".repeat(100)));
            }
        }
    }
}
