use digit_sequence::*;
use pretty_assertions::assert_eq;

#[test]
fn convert_from_valid_unsigned() {
    let sequence: DigitSequence = 9081u16.into();

    assert_eq!(sequence, [9, 0, 8, 1]);
}

#[test]
fn convert_from_valid_signed() {
    let sequence: DigitSequence = 9081i16.try_into().unwrap();

    assert_eq!(sequence, [9, 0, 8, 1]);
}

#[test]
fn convert_from_valid_numeric_array_slice() {
    let source = [0, 1, 0, 7];
    let slice: &[u8] = &source;
    let sequence: DigitSequence = slice.try_into().unwrap();

    assert_eq!(sequence, source);
}

#[test]
fn convert_from_valid_array_literal_reference() {
    let source = [0, 1, 0, 7];
    let sequence: DigitSequence = (&source).try_into().unwrap();

    assert_eq!(sequence, source);
}

#[test]
fn convert_from_valid_numeric_vector_reference() {
    let source = vec![0, 1, 0, 3];
    let sequence: DigitSequence = (&source).try_into().unwrap();

    assert_eq!(sequence, source);
}

#[test]
fn convert_from_valid_string_literal() {
    let sequence: DigitSequence = "01294860".parse().unwrap();

    assert_eq!(sequence, [0, 1, 2, 9, 4, 8, 6, 0]);
}
