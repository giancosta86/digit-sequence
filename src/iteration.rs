use crate::DigitSequence;

impl DigitSequence {
    /// Repeatable iteration over references to the digits.
    ///
    /// ```
    /// use digit_sequence::*;
    ///
    /// # fn main() -> GenericResult<()> {
    /// let source = [9, 5, 0, 2];
    /// let sequence: DigitSequence = (&source).try_into()?;
    /// let mut target: Vec<u8> = vec![];
    ///
    /// for &digit in sequence.iter() {
    ///     target.push(digit)
    /// }
    ///
    /// for &digit in sequence.iter() {
    ///     target.push(digit)
    /// }
    ///
    /// let expected_vec: Vec<u8> = [&source[..], &source[..]].concat();
    ///
    /// assert_eq!(target, expected_vec);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }
}

/// Consuming iteration on [DigitSequence] is supported:
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let source = [9, 5, 0, 2];
/// let sequence: DigitSequence = (&source).try_into()?;
/// let mut target: Vec<u8> = vec![];
///
/// for digit in sequence {
///     target.push(digit)
/// }
///
/// assert_eq!(target, source);
///
/// # Ok(())
/// # }
/// ```
///
/// In this case, the sequence cannot be reused once consumed:
///
/// ```compile_fail
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let source = [9, 5, 0, 2];
/// let sequence: DigitSequence = (&source).try_into()?;
///
/// for digit in sequence {
///     //Just do something
/// }
///
/// for digit in sequence {
///     //Could never arrive here
/// }
///
/// # Ok(())
/// # }
/// ```
impl IntoIterator for DigitSequence {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Repeatable, reference-based iteration on &[DigitSequence] is supported.
///
/// ```
/// use digit_sequence::*;
///
/// # fn main() -> GenericResult<()> {
/// let source = [9, 5, 0, 2];
/// let sequence: DigitSequence = (&source).try_into()?;
/// let mut target: Vec<u8> = vec![];
///
/// for digit in &sequence {
///     target.push(*digit)
/// }
///
/// for digit in &sequence {
///     target.push(*digit)
/// }
///
/// let expected_vec: Vec<u8> = [&source[..], &source[..]].concat();
///
/// assert_eq!(target, expected_vec);
///
/// # Ok(())
/// # }
/// ```
impl<'a> IntoIterator for &'a DigitSequence {
    type Item = &'a u8;

    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
