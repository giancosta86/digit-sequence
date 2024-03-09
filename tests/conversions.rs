use digit_sequence::*;
use pretty_assertions::assert_eq as eq;
use speculate2::*;

speculate! {
    describe "Converting to digit sequence" {
        describe "from a valid unsigned" {
            it "should work" {
                let sequence = DigitSequence::from(9081u16);

                eq!(sequence, [9, 0, 8, 1]);
            }
        }

        describe "from a valid signed" {
            it "should work" {
                let sequence = DigitSequence::try_from(9081i16).unwrap();

                eq!(sequence, [9, 0, 8, 1]);
            }
        }

        describe "from a valid numeric array slice" {
            it "should work" {
                let source = [0, 1, 0, 7];
                let slice: &[u8] = &source;
                let sequence = DigitSequence::try_from(slice).unwrap();

                eq!(sequence, source);
            }
        }

        describe "from a reference to a valid array literal" {
            it "should work" {
                let source = [0, 1, 0, 7];
                let sequence = DigitSequence::try_from(&source).unwrap();

                eq!(sequence, source);
            }
        }

        describe "from a valid numeric vector reference" {
            it "should work" {
                let source = vec![0, 1, 0, 3];
                let sequence = DigitSequence::try_from(&source).unwrap();

                eq!(sequence, source);
            }
        }

        describe "from a valid string literal" {
            it "should work" {
                let sequence: DigitSequence = "01294860".parse().unwrap();

                eq!(sequence, [0, 1, 2, 9, 4, 8, 6, 0]);
            }
        }
    }
}
