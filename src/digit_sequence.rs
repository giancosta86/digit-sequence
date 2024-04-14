use crate::{CrateError, CrateResult};
use std::fmt::Display;

/// Immutable sequence of u8 digits.
///
/// # Creation
///
/// A digit sequence is usually created via *conversions* - which are not always *infallible*; a [constructor](DigitSequence::new) is also provided to instantiate
/// an empty sequence.
///
/// ## Infallible conversions
///
/// * from any **unsigned** number:
///
///   ```
///   use digit_sequence::{CrateResult, DigitSequence};
///
///   # fn main() -> CrateResult<()> {
///   let sequence: DigitSequence = 985u32.into();
///
///   assert_eq!(sequence, [9, 8, 5]);
///   # Ok(())
///   # }
///   ```
///
/// ## Fallible conversions
///
/// * from any **signed** number - which fails if the number is negative:
///
///   ```
///   use digit_sequence::{CrateResult, DigitSequence};
///
///   # fn main() -> CrateResult<()> {
///   let sequence: DigitSequence = 3791i32.try_into()?;
///
///   assert_eq!(sequence, [3, 7, 9, 1]);
///   # Ok(())
///   # }
///   ```
///
/// * from &[str] - which fails in case of non-digit (radix 10) characters:
///  
///   ```
///   use digit_sequence::{CrateResult, DigitSequence};
///
///   # fn main() -> CrateResult<()> {
///   let sequence: DigitSequence = "09240".parse()?;
///
///   assert_eq!(sequence, [0, 9, 2, 4, 0]);
///   # Ok(())
///   # }
///   ```
///
/// * from a &[[u8]] - which fails if the numbers are not 0-9 digits:
///
///   ```
///   use digit_sequence::{CrateResult, DigitSequence};
///
///   # fn main() -> CrateResult<()> {
///   let source = [0, 9, 2, 4, 0];
///   let sequence: DigitSequence = (&source).try_into()?;
///
///   assert_eq!(sequence, source);
///   # Ok(())
///   # }
///   ```
///
/// * to any **unsigned** number - which can fail with [CrateError::Overflow]:
///
///   ```
///   use digit_sequence::{CrateResult, CrateError, DigitSequence};
///
///   # fn main() -> CrateResult<()> {
///   let in_range_sequence: DigitSequence = "90".parse()?;
///   let number: u128 = in_range_sequence.try_into()?;
///   assert_eq!(number, 90);
///
///   let overflowing_sequence: DigitSequence = "1".repeat(100).parse()?;
///   let result: CrateResult<u128> = overflowing_sequence.try_into();
///   assert_eq!(result, Err(CrateError::Overflow));
///
///   # Ok(())
///   # }
///   ```
///
///   Conversion from &DigitSequence is also supported:
///
///   ```
///   use digit_sequence::{CrateResult, CrateError, DigitSequence};
///
///   # fn main() -> CrateResult<()> {
///   let sequence: DigitSequence = "90".parse()?;
///   let reference = &sequence;  
///   let number: u128 = reference.try_into()?;
///   assert_eq!(number, 90);
///
///   # Ok(())
///   # }
///   ```
///
/// # Usage
///
/// This data structure implements [IntoIterator] and also provides the [iter](Self::iter) method - thus enabling standard iterations as well as the [Iterator] methods.
///
/// ## `for`-iterations
///  
/// ### read-only
///
/// ```
/// use digit_sequence::DigitSequence;
///
/// let sequence: DigitSequence = 1234567890u128.into();
///   
/// let mut even_vec = vec![];
///
/// for &digit in &sequence {
///   if digit % 2 == 0 {
///     even_vec.push(digit);
///   }
/// }
///
/// assert_eq!(even_vec, vec![2, 4, 6, 8, 0]);
/// ```
///
/// ### consuming
///   
/// ```
/// use digit_sequence::DigitSequence;
///
/// let sequence: DigitSequence = 1234567890u128.into();
///   
/// let mut even_vec = vec![];
///
/// for digit in sequence {
///   if digit % 2 == 0 {
///     even_vec.push(digit);
///   }
/// }
///
/// assert_eq!(even_vec, vec![2, 4, 6, 8, 0]);
/// ```
///
/// ## [Iterator] methods
///
/// ### read-only
///
/// ```
/// use digit_sequence::DigitSequence;
///
/// let sequence: DigitSequence = 1234567890u128.into();
///
/// let even_vec: Vec<u8> = sequence.iter()
///   .filter(|&digit| digit % 2 == 0)
///   .map(|&digit|digit)
///   .collect();
///
/// assert_eq!(even_vec, vec![2, 4, 6, 8, 0]);
/// ```
///
/// ### consuming
/// ```
/// use digit_sequence::DigitSequence;
///
/// let sequence: DigitSequence = 1234567890u128.into();
///
/// let even_vec: Vec<u8> = sequence.into_iter()
///   .filter(|digit| digit % 2 == 0)
///   .collect();
///
/// assert_eq!(even_vec, vec![2, 4, 6, 8, 0]);
/// ```
///
/// # Conversion to String
///
/// * the [Debug] trait is derived
///
/// * for the [Display] trait, the sequence is formatted just by joining its digits
///  
/// # Serialization
///
/// When the `serde` feature is enabled for this crate, [Self] implements the [serde::Serialize] and [serde::Deserialize] traits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DigitSequence(Vec<u8>);

pub fn internal_create_digit_sequence(digits: Vec<u8>) -> DigitSequence {
    DigitSequence(digits)
}

impl DigitSequence {
    /// Creates an empty DigitSequence
    ///
    /// ```
    /// use digit_sequence::DigitSequence;
    ///
    /// let sequence = DigitSequence::new();
    ///
    /// assert_eq!(sequence.iter().len(), 0);
    /// ```
    pub fn new() -> DigitSequence {
        DigitSequence(vec![])
    }

    /// Convenience method for iterating over references to the digits.
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }
}

impl TryFrom<&[u8]> for DigitSequence {
    type Error = CrateError;

    fn try_from(digits: &[u8]) -> CrateResult<Self> {
        let mut digits_vec = Vec::new();

        for &digit in digits.iter() {
            if digit >= 10 {
                return Err(CrateError::NonDigitNumber(digit as u128));
            }

            digits_vec.push(digit);
        }

        Ok(DigitSequence(digits_vec))
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for DigitSequence {
    type Error = CrateError;

    fn try_from(digits: &[u8; N]) -> CrateResult<Self> {
        Self::try_from(digits as &[u8])
    }
}

impl TryFrom<&Vec<u8>> for DigitSequence {
    type Error = CrateError;

    fn try_from(value: &Vec<u8>) -> CrateResult<Self> {
        Self::try_from(value as &[u8])
    }
}

impl Display for DigitSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.0 {
            write!(f, "{}", digit)?;
        }

        Ok(())
    }
}

impl<const N: usize> PartialEq<[u8; N]> for DigitSequence {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&[u8]> for DigitSequence {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for DigitSequence {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Vec<u8>> for DigitSequence {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.0 == *other
    }
}

impl IntoIterator for DigitSequence {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a DigitSequence {
    type Item = &'a u8;

    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Digit sequence" {
            it "should be debuggable" {
                let sequence = DigitSequence::try_from(&[9, 2, 5, 8]).unwrap();

                eq!(format!("{:?}", sequence), "DigitSequence([9, 2, 5, 8])");
            }


            describe "construction" {
                describe "when passing an empty array slice" {
                    it "should create an empty sequence" {
                        let sequence = DigitSequence::try_from(&[]).unwrap();

                        eq!(sequence.iter().len(), 0)
                    }
                }

                describe "when passing a valid array slice" {
                    it "should keep the length" {
                        let sequence = DigitSequence::try_from(&[9, 3, 7, 1, 8]).unwrap();

                        eq!(sequence.iter().len(), 5);
                    }

                    it "should create an equivalent sequence" {
                        let source_array = [9, 3, 7, 1, 8];
                        let sequence = DigitSequence::try_from(&source_array).unwrap();

                        eq!(sequence, source_array);
                    }
                }

                describe "when passing an invalid array slice" {
                    it "should return Err" {
                        let result = DigitSequence::try_from(&[9, 3, 18]);

                        eq!(result, Err(CrateError::NonDigitNumber(18)));
                    }
                }

                describe "when passing a valid vector reference" {
                    it "should create an equivalent sequence" {
                        let source_vec: Vec<u8> = vec![8, 0, 1, 3, 5];

                        let sequence = DigitSequence::try_from(&source_vec).unwrap();

                        eq!(sequence, source_vec);
                    }
                }

                describe "when passing a reference to an empty array" {
                    it "should work" {
                        let source = [];
                        let sequence = DigitSequence::try_from(&source).unwrap();

                        eq!(sequence, source);
                    }
                }
            }


            describe "equality" {
                describe "with another instance" {
                    it "should work" {
                        let source = [1, 3, 5, 7, 9];

                        let sequence_1 = DigitSequence::try_from(&source).unwrap();
                        let sequence_2 = DigitSequence::try_from(&source).unwrap();

                        eq!(sequence_1, sequence_2);
                    }
                }

                describe "with a vector" {
                    it "should work" {
                        let source = [1, 3, 5, 7, 9];

                        let sequence = DigitSequence::try_from(&source).unwrap();
                        let vector = Vec::from(&source);

                        eq!(sequence, vector);
                    }
                }

                describe "with an array slice" {
                    it "should work" {
                        let source = [1, 3, 5, 7, 9];

                        let sequence = DigitSequence::try_from(&source).unwrap();

                        eq!(sequence, &source);
                    }
                }

                describe "with an array" {
                    it "should work" {
                        let source = [1, 3, 5, 7, 9];

                        let sequence = DigitSequence::try_from(&source).unwrap();

                        eq!(sequence, source);
                    }
                }
            }


            describe "for-iteration" {
                describe "by default" {
                    it "should consume the object" {
                        let source = [9, 5, 0, 2];
                        let sequence = DigitSequence::try_from(&source).unwrap();
                        let mut target: Vec<u8> = vec![];

                        for digit in sequence {
                            target.push(digit)
                        }

                        eq!(target, source);
                    }
                }

                describe "on reference" {
                    it "should be repeatable" {
                        let source = [9, 5, 0, 2];
                        let sequence = DigitSequence::try_from(&source).unwrap();
                        let mut target: Vec<u8> = vec![];

                        for digit in &sequence {
                            target.push(*digit)
                        }

                        for digit in &sequence {
                            target.push(*digit)
                        }

                        let expected_vec: Vec<u8> = [&source[..], &source[..]].concat();

                        eq!(target, expected_vec);
                    }
                }

                describe "via iter()" {
                    it "should be repeatable" {
                        let source = [9, 5, 0, 2];
                        let sequence = DigitSequence::try_from(&source).unwrap();
                        let mut target: Vec<u8> = vec![];

                        for &digit in sequence.iter() {
                            target.push(digit)
                        }

                        for &digit in sequence.iter() {
                            target.push(digit)
                        }

                        let expected_vec: Vec<u8> = [&source[..], &source[..]].concat();

                        eq!(target, expected_vec);
                    }
                }
            }


            describe "conversion to string" {
                fn test_case(digits: &[u8], expected_string: &str) {
                    let digit_sequence = DigitSequence::try_from(digits).unwrap();

                    eq!(&digit_sequence.to_string(), expected_string);
                }

                describe "when the sequence is empty" {
                    it "should return an empty string" {
                        test_case(&[], "");
                    }
                }

                describe "when the sequence contains one digit" {
                    it "should return the digit as a string" {
                        test_case(&[9], "9");
                    }
                }

                describe "when the sequence contains multiple digits" {
                    it "should return a string of digits" {
                        test_case(&[1, 7, 5, 4, 3, 8], "175438");
                    }
                }

            }


            describe "default" {
                it "should be an empty sequence" {
                    eq!(DigitSequence::default(), []);
                }
            }


            describe "ordering" {
                it "should work based on values" {
                    let sequence_1 = DigitSequence::try_from(&[9]).unwrap();
                    let sequence_2 = DigitSequence::try_from(&[4]).unwrap();

                    assert!(sequence_1 > sequence_2);
                }

                it "should work based on length" {
                    let sequence_1 = DigitSequence::try_from(&[9]).unwrap();
                    let sequence_2 = DigitSequence::try_from(&[9, 7]).unwrap();

                    assert!(sequence_1 < sequence_2);
                }
            }
        }
    }

    #[cfg(feature = "serde")]
    speculate! {
        describe "JSON serialization" {
            before {
                let original_sequence: DigitSequence = 9786u16.into();
                let expected_json = "[9,7,8,6]";
            }

            it "should serialize" {
                let json = serde_json::to_string(&original_sequence).unwrap();

                eq!(json, expected_json);
            }

            it "should deserialize" {
                let deserialized: DigitSequence = serde_json::from_str(expected_json).unwrap();

                eq!(deserialized, original_sequence);
            }
        }
    }
}
