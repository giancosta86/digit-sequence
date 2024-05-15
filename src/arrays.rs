use crate::{CrateError, CrateResult, DigitSequence};

impl<const N: usize> TryFrom<&[u8; N]> for DigitSequence {
    type Error = CrateError;

    fn try_from(digits: &[u8; N]) -> CrateResult<Self> {
        Self::try_from(digits as &[u8])
    }
}

impl<const N: usize> PartialEq<[u8; N]> for DigitSequence {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for DigitSequence {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.0 == *other
    }
}
