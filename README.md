# endi

[![Build Status](https://github.com/zeenix/endi/actions/workflows/rust.yml/badge.svg)](https://github.com/zeenix/endi/actions/workflows/rust.yml)

Yet another endian handling library for Rust. The approach is very similar to that of
`byteordered` crate with its `Endianness` enum, except that `endi` is much simpler and doesn't
depend on `byteorder` (or anything at all).

## Usage

The main type is `Endianness` enum which can be either `Big` or `Little`. It provides various 
methods to read and write integers of different sizes and endianness.

```rust
use endi::{Endianness, ReadBytes, WriteBytes};

let mut buf = [0u8; 4];
let le = Endianness::Little;
le.write_u32(&mut buf, 0xAB_BA_FE_EF);
assert_eq!(le.read_u32(&buf), 0xAB_BA_FE_EF);

// Using the `ReadBytes` and `WriteBytes` traits:
let mut cursor = std::io::Cursor::new(&mut buf[..]);
le.write_u32(&mut cursor, 0xAB_BA_FE_EF);
assert_eq!(le.read_u32(&mut cursor), 0xAB_BA_FE_EF);
```

## nostd

You can disable `std` by disabling the default `std` feature. This will disable the `ReadBytes` and
`WriteBytes` traits.

## License

[MIT](LICENSE)
