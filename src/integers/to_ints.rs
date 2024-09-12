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
    use pretty_assertions::assert_eq;

    #[test]
    fn roundtrip_convert_0() {
        expect_try_roundtrip_conversion(0u8);
    }

    #[test]
    fn roundtrip_convert_u8() {
        expect_try_roundtrip_conversion(90u8);
    }

    #[test]
    fn roundtrip_convert_u16() {
        expect_try_roundtrip_conversion(90u16);
    }

    #[test]
    fn roundtrip_convert_u32() {
        expect_try_roundtrip_conversion(90u32);
    }

    #[test]
    fn roundtrip_convert_u64() {
        expect_try_roundtrip_conversion(90u64);
    }

    #[test]
    fn roundtrip_convert_u128() {
        expect_try_roundtrip_conversion(90u128);
    }

    #[test]
    fn roundtrip_convert_usize() {
        expect_try_roundtrip_conversion(90usize);
    }

    #[test]
    fn convert_u128_max() {
        expect_try_roundtrip_conversion(u128::MAX);
    }

    fn expect_unsigned_overflow(source: &str) {
        let sequence: DigitSequence = source.parse().unwrap();
        let conversion_result: CrateResult<u128> = sequence.try_into();

        assert_eq!(conversion_result, Err(CrateError::Overflow));
    }

    #[test]
    fn convert_u128_max_plus_one_to_u128() {
        expect_unsigned_overflow("340282366920938463463374607431768211456");
    }

    #[test]
    fn convert_huge_sequence_of_1s_to_u128() {
        expect_unsigned_overflow("1".repeat(100).as_str());
    }

    #[test]
    fn convert_a_1_of_huge_magnitude_to_u128() {
        expect_unsigned_overflow(&format!("1{}", "0".repeat(100)));
    }

    #[test]
    fn roundtrip_convert_0_via_ref() {
        expect_try_roundtrip_conversion_via_ref(0u8);
    }

    #[test]
    fn roundtrip_convert_u8_via_ref() {
        expect_try_roundtrip_conversion_via_ref(90u8);
    }

    #[test]
    fn roundtrip_convert_u16_via_ref() {
        expect_try_roundtrip_conversion_via_ref(90u16);
    }

    #[test]
    fn roundtrip_convert_u32_via_ref() {
        expect_try_roundtrip_conversion_via_ref(90u32);
    }

    #[test]
    fn roundtrip_convert_u64_via_ref() {
        expect_try_roundtrip_conversion_via_ref(90u64);
    }

    #[test]
    fn roundtrip_convert_u128_via_ref() {
        expect_try_roundtrip_conversion_via_ref(90u128);
    }

    #[test]
    fn roundtrip_convert_usize_via_ref() {
        expect_try_roundtrip_conversion_via_ref(90usize);
    }

    #[test]
    fn convert_u128_max_via_ref() {
        expect_try_roundtrip_conversion_via_ref(u128::MAX);
    }

    fn expect_unsigned_overflow_via_ref(source: &str) {
        let sequence: DigitSequence = source.parse().unwrap();
        let reference = &sequence;
        let conversion_result: CrateResult<u128> = reference.try_into();

        assert_eq!(conversion_result, Err(CrateError::Overflow));
    }

    #[test]
    fn convert_u128_max_plus_one_to_u128_via_ref() {
        expect_unsigned_overflow_via_ref("340282366920938463463374607431768211456");
    }

    #[test]
    fn convert_huge_sequence_of_1s_to_u128_via_ref() {
        expect_unsigned_overflow_via_ref(&"1".repeat(100));
    }

    #[test]
    fn convert_a_1_of_huge_magnitude_to_u128_via_ref() {
        expect_unsigned_overflow_via_ref(&format!("1{}", "0".repeat(100)));
    }
}
