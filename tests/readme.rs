//
// CODE FROM README
//

use digit_sequence::*;

fn main() -> GenericResult<()> {
    assert_eq!(DigitSequence::new(), []);
    assert_eq!(DigitSequence::default(), []);

    let sequence: DigitSequence = [3, 8, 7].try_into()?;
    assert_eq!(sequence, [3, 8, 7]);

    assert_eq!(format!("{:?}", sequence), "DigitSequence([3, 8, 7])");
    assert_eq!(sequence.to_string(), "387");

    let mapped_vec: Vec<u8> = sequence.iter().map(|digit| digit + 1).collect();
    assert_eq!(mapped_vec, vec![4, 9, 8]);

    let number: u16 = sequence.try_into()?;
    assert_eq!(number, 387);

    Ok(())
}

// Test
//
use speculate2::*;

speculate! {
    describe "The source code in the README file" {
        it "must compile and run" {
            main().unwrap();
        }
    }
}
