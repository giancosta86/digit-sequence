# digit-sequence

_Sequence of u8 digits_

[![CI](https://github.com/giancosta86/digit-sequence/actions/workflows/publish-to-crates.yml/badge.svg)](https://github.com/giancosta86/digit-sequence/actions/workflows/publish-to-crates.yml)
![Crates.io Version](https://img.shields.io/crates/v/digit_sequence?style=flat&logo=rust)

This Rust crate revolves around the `DigitSequence` struct, a sequence of 0-9 `u8` digits, with:

- conversions from integers, numeric sequences and strings

- different iteration strategies

- a custom `Result` and a custom `Error`.

- optional `serde` I/O

```rust
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
```

## Features

This crate supports the following _optional_ features:

- `serde`: enables JSON conversions via [serde](https://crates.io/crates/serde)

## Crates.io

https://crates.io/crates/digit-sequence

## Documentation

https://docs.rs/digit-sequence

## License

[MIT](LICENSE)
