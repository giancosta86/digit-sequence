# digit-sequence

_Sequence of u8 digits_

[![CI](https://github.com/giancosta86/digit-sequence/actions/workflows/publish-to-crates.yml/badge.svg)](https://github.com/giancosta86/digit-sequence/actions/workflows/publish-to-crates.yml)

This Rust crate revolves around `DigitSequence`, a sequence of 0-9 `u8` digits, with:

- conversions from integers, numeric sequences and strings

- different iteration strategies

- a custom `Result` and a custom `Error`.

- optional `serde` I/O

## Optional features

### serde

To enable I/O via `serde`:

```toml
[dependencies.digit-sequence]
version = "*"
features = ["serde"]
```

## Crates.io

https://crates.io/crates/digit-sequence

## Documentation

https://docs.rs/digit-sequence

## License

[MIT](LICENSE)
