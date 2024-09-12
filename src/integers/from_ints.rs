use crate::{CrateError, CrateResult, DigitSequence};
use std::collections::LinkedList;

macro_rules! impl_try_from_signed {
    ($type: ty) => {
        /// Conversion from a *signed* integer to a [DigitSequence]
        /// is fallible - in particular, *negative* values
        /// result in [CrateError::NegativeNumber].
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
        /// Conversion from an *unsigned* integer to a [DigitSequence]
        /// is always infallible.
        impl From<$type> for DigitSequence {
            fn from(value: $type) -> DigitSequence {
                convert_from_positive!(value)
            }
        }
    };
}

macro_rules! convert_from_positive {
    ($value: ident) => {{
        let mut result = LinkedList::new();
        let mut current_value = $value;

        loop {
            let digit = current_value % 10;

            result.push_front(digit as u8);

            current_value /= 10;

            if current_value == 0 {
                break;
            }
        }

        DigitSequence(result.into_iter().collect())
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
    use crate::test_utils::{expect_digits_from, expect_try_digits_from};
    use pretty_assertions::assert_eq;

    #[test]
    fn convert_0() {
        let digit_sequence: DigitSequence = 0u8.into();

        assert_eq!(digit_sequence, [0]);
    }

    #[test]
    fn convert_negative_number() {
        let result: CrateResult<DigitSequence> = (-4).try_into();

        assert_eq!(result, Err(CrateError::NegativeNumber(-4)));
    }

    #[test]
    fn convert_u8() {
        expect_digits_from(107u8, &[1, 0, 7])
    }

    #[test]
    fn convert_u16() {
        expect_digits_from(107u16, &[1, 0, 7])
    }

    #[test]
    fn convert_u32() {
        expect_digits_from(107u32, &[1, 0, 7])
    }

    #[test]
    fn convert_u64() {
        expect_digits_from(107u64, &[1, 0, 7])
    }

    #[test]
    fn convert_u128() {
        expect_digits_from(107u128, &[1, 0, 7])
    }

    #[test]
    fn convert_usize() {
        expect_digits_from(107usize, &[1, 0, 7])
    }

    #[test]
    fn convert_i8() {
        expect_try_digits_from(107i8, &[1, 0, 7])
    }

    #[test]
    fn convert_i16() {
        expect_try_digits_from(107i16, &[1, 0, 7])
    }

    #[test]
    fn convert_i32() {
        expect_try_digits_from(107i32, &[1, 0, 7])
    }

    #[test]
    fn convert_i64() {
        expect_try_digits_from(107i64, &[1, 0, 7])
    }

    #[test]
    fn convert_i128() {
        expect_try_digits_from(107i128, &[1, 0, 7])
    }

    #[test]
    fn convert_isize() {
        expect_try_digits_from(107isize, &[1, 0, 7])
    }
}
