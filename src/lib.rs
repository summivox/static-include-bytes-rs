#![no_std]
#![doc = include_str!("../README.md")]

/// Inserts a static byte array definition with both its content and length loaded from a file at
/// compile time. `static_include_bytes!(NAME, PATH)` is semantically equivalent to
/// `static NAME: [u8; _] = *include_bytes!(PATH);` which you cannot write directly as of 2023-08-18
/// due to <https://github.com/rust-lang/rust/issues/85077> .
///
/// # Example
///
/// ```rust
/// use static_include_bytes::static_include_bytes;
///
/// static_include_bytes!(TEN_BYTES, concat!(env!("CARGO_MANIFEST_DIR"), "/ten_bytes.bin"));
///
/// assert_eq!(TEN_BYTES.len(), 10);
/// assert_eq!(&TEN_BYTES, b"0123456789");
/// ```
///
#[macro_export]
macro_rules! static_include_bytes {
    ($name:ident, $path:expr) => {
        static $name: [u8; include_bytes!($path).len()] = *include_bytes!($path);
    };
}
