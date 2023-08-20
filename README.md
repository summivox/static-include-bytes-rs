# `static_include_bytes!` Macro

[![Crates.io](https://img.shields.io/crates/v/static-include-bytes)](https://crates.io/crates/static-include-bytes)
[![docs.rs](https://img.shields.io/docsrs/static-include-bytes)](https://docs.rs/static-include-bytes)

Like the built-in `include_bytes!` macro, but produces a static array definition.

## Example

```rust
use static_include_bytes::static_include_bytes;

static_include_bytes!(#[no_mangle] TEN_BYTES = concat!(env!("CARGO_MANIFEST_DIR"), "/ten_bytes.bin"));

assert_eq!(TEN_BYTES.len(), 10);
assert_eq!(&TEN_BYTES, b"0123456789");
```

The macro invocation is equivalent to `static TEN_BYTES: [u8; _] = include_bytes!(...);`, but as of 2023-08-18, you cannot write this directly due to <https://github.com/rust-lang/rust/issues/85077> .