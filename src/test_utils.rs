use crate::DigitSequence;
use core::fmt::Debug;
use pretty_assertions::assert_eq;
use std::error::Error;

pub fn expect_digits_from<T: Into<DigitSequence>>(source: T, expected_digits: &[u8]) {
    let sequence: DigitSequence = source.into();

    assert_eq!(sequence, expected_digits);
}

pub fn expect_try_digits_from<T: TryInto<DigitSequence>>(source: T, expected_digits: &[u8])
where
    <T as TryInto<DigitSequence>>::Error: Debug,
{
    let actual_sequence: DigitSequence = source.try_into().unwrap();

    assert_eq!(actual_sequence, expected_digits);
}

pub fn expect_try_roundtrip_conversion<E, T>(source: T)
where
    E: Error,
    T: Into<DigitSequence> + TryFrom<DigitSequence, Error = E> + PartialEq<T> + Debug + Copy,
{
    let sequence: DigitSequence = source.into();
    let roundtrip: T = sequence.try_into().unwrap();

    assert_eq!(roundtrip, source);
}

pub fn expect_try_roundtrip_conversion_via_ref<E, T>(source: T)
where
    E: Error,
    T: Into<DigitSequence>
        + for<'a> TryFrom<&'a DigitSequence, Error = E>
        + PartialEq<T>
        + Debug
        + Copy,
{
    let sequence: DigitSequence = source.into();
    let reference = &sequence;
    let roundtrip: T = reference.try_into().unwrap();

    assert_eq!(roundtrip, source);
}
