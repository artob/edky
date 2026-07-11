// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// Errors when parsing a public key from a hexadecimal input string.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ParsePublicKeyError {
    /// The string was not exactly 64 characters long.
    #[error("hexadecimal public keys must be exactly 64 characters long")]
    InvalidLength(usize),

    /// The string contained non-ASCII bytes.
    #[error("hexadecimal public keys cannot contain non-ASCII characters")]
    InvalidChars,

    /// The string contained non-hexadecimal characters.
    #[error("hexadecimal public keys can only contain characters in [0-9a-fA-F]")]
    InvalidDigit(usize),
}
