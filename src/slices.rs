use crate::{CrateError, CrateResult, DigitSequence};

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

impl PartialEq<&[u8]> for DigitSequence {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}
