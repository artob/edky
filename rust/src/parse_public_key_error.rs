// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// Errors when parsing a public key from a hexadecimal input string.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ParsePublicKeyError {
    /// The string was not exactly 64 characters long.
    #[error("invalid public key length")]
    InvalidLength(usize),

    /// The string did not start with the expected prefix.
    #[error("invalid public key prefix")]
    InvalidPrefix,

    /// The string did not end with the expected suffix.
    #[error("invalid public key suffix")]
    InvalidSuffix,

    /// The string contained non-ASCII bytes.
    #[error("invalid public key character")]
    InvalidChars,

    /// The string contained non-hexadecimal characters.
    #[error("invalid public key digit")]
    InvalidDigit(usize),
}

impl<T> From<&T> for ParsePublicKeyError
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

#[cfg(feature = "base58")]
impl From<bs58::decode::Error> for ParsePublicKeyError {
    fn from(input: bs58::decode::Error) -> Self {
        #![allow(non_snake_case)]
        use bs58::decode::Error::*;
        match input {
            BufferTooSmall => Self::InvalidLength(0),
            InvalidCharacter => Self::InvalidDigit(0),
            NonAsciiCharacter => Self::InvalidChars,
        }
    }
}

#[cfg(feature = "base64")]
impl From<data_encoding::DecodeError> for ParsePublicKeyError {
    fn from(input: data_encoding::DecodeError) -> Self {
        use data_encoding::DecodeKind::*;
        match input.kind {
            Length => Self::InvalidLength(input.position),
            Symbol => Self::InvalidDigit(input.position),
            Trailing => Self::InvalidChars,
            Padding => Self::InvalidLength(input.position),
        }
    }
}

#[cfg(feature = "base64")]
impl From<data_encoding::DecodePartial> for ParsePublicKeyError {
    fn from(input: data_encoding::DecodePartial) -> Self {
        input.error.into()
    }
}

#[cfg(feature = "clientele")]
impl From<ParsePublicKeyError> for clientele::SysexitsError {
    fn from(_: ParsePublicKeyError) -> Self {
        clientele::SysexitsError::EX_DATAERR
    }
}
