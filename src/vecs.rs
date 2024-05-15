use crate::{CrateError, CrateResult, DigitSequence};

impl TryFrom<&Vec<u8>> for DigitSequence {
    type Error = CrateError;

    fn try_from(value: &Vec<u8>) -> CrateResult<Self> {
        Self::try_from(value as &[u8])
    }
}

impl PartialEq<Vec<u8>> for DigitSequence {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.0 == *other
    }
}
