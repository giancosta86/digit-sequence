use crate::DigitSequence;
use core::fmt::Debug;
use pretty_assertions::assert_eq as eq;
use std::error::Error;

pub fn test_roundtrip_conversion<E, T>(source: T)
where
    E: Error,
    T: Into<DigitSequence> + TryFrom<DigitSequence, Error = E> + PartialEq<T> + Debug + Copy,
{
    let sequence: DigitSequence = source.into();
    let roundtrip: T = sequence.try_into().unwrap();

    eq!(roundtrip, source);
}

pub fn test_roundtrip_conversion_via_ref<E, T>(source: T)
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

    eq!(roundtrip, source);
}
