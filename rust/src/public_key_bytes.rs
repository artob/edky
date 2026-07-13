// This is free and unencumbered software released into the public domain.

use super::{ParsePublicKeyError, PublicKeyEncoding};
use core::{fmt::Display, ops::Deref, str::FromStr};

pub const PUBLIC_KEY_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "alloc::string::String", into = "alloc::string::String")
)]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromBytes, zerocopy::IntoBytes))]
pub struct PublicKeyBytes(pub(crate) [u8; PUBLIC_KEY_LEN]);

impl Display for PublicKeyBytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
        let mut buffer = [0u8; PUBLIC_KEY_LEN * 2];
        for (i, &byte) in self.0.iter().enumerate() {
            buffer[i * 2] = HEX_CHARS[(byte >> 4) as usize];
            buffer[i * 2 + 1] = HEX_CHARS[(byte & 0x0f) as usize];
        }
        // Safe to unwrap as it's an ASCII buffer containing only HEX_CHARS:
        f.write_str(core::str::from_utf8(&buffer).unwrap())
    }
}

impl FromStr for PublicKeyBytes {
    type Err = ParsePublicKeyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != PUBLIC_KEY_LEN * 2 {
            return Err(ParsePublicKeyError::InvalidLength(input.len()));
        }
        if !input.is_ascii() {
            return Err(ParsePublicKeyError::InvalidChars);
        }

        let mut result = [0u8; PUBLIC_KEY_LEN];

        for (i, chunk) in input.as_bytes().chunks(2).enumerate() {
            if chunk.iter().any(|c| !c.is_ascii_hexdigit()) {
                return Err(ParsePublicKeyError::InvalidDigit(i * 2));
            }
            // Safe to unwrap as chunk is ASCII and contains only HEX_CHARS:
            let digit = core::str::from_utf8(chunk).unwrap();
            result[i] = u8::from_str_radix(digit, 16).unwrap();
        }

        Ok(Self(result))
    }
}

impl PublicKeyBytes {
    pub const ZERO: Self = Self([0u8; PUBLIC_KEY_LEN]);

    pub fn decode(
        encoding: PublicKeyEncoding,
        input: impl AsRef<str>,
    ) -> Result<Self, ParsePublicKeyError> {
        crate::decode(encoding, input)
    }

    #[cfg(feature = "alloc")]
    pub fn encode(&self, encoding: PublicKeyEncoding) -> Option<alloc::string::String> {
        crate::encode(encoding, self)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, ParsePublicKeyError> {
        if slice.len() != PUBLIC_KEY_LEN {
            return Err(ParsePublicKeyError::InvalidLength(slice.len()));
        }
        let mut buffer = [0u8; PUBLIC_KEY_LEN];
        buffer.copy_from_slice(slice);
        Ok(Self(buffer))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LEN] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_LEN] {
        self.0
    }

    pub fn into_bytes(self) -> [u8; PUBLIC_KEY_LEN] {
        self.0
    }
}

impl AsRef<[u8]> for PublicKeyBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl AsRef<[u8; PUBLIC_KEY_LEN]> for PublicKeyBytes {
    fn as_ref(&self) -> &[u8; PUBLIC_KEY_LEN] {
        &self.0
    }
}

impl Deref for PublicKeyBytes {
    type Target = [u8; PUBLIC_KEY_LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<&T> for PublicKeyBytes
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<[u8; PUBLIC_KEY_LEN]> for PublicKeyBytes {
    fn from(input: [u8; PUBLIC_KEY_LEN]) -> Self {
        Self(input)
    }
}

impl TryFrom<&[u8]> for PublicKeyBytes {
    type Error = ParsePublicKeyError;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(input)
    }
}

#[cfg(feature = "alloc")]
impl From<&alloc::vec::Vec<u8>> for PublicKeyBytes {
    fn from(input: &alloc::vec::Vec<u8>) -> Self {
        let mut bytes = [0u8; PUBLIC_KEY_LEN];
        let len = bytes.len().min(input.len());
        bytes[..len].copy_from_slice(&input[..len]);
        Self(bytes)
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::string::String> for PublicKeyBytes {
    type Error = ParsePublicKeyError;

    fn try_from(input: alloc::string::String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

#[cfg(feature = "alloc")]
impl From<PublicKeyBytes> for alloc::string::String {
    fn from(input: PublicKeyBytes) -> alloc::string::String {
        alloc::string::ToString::to_string(&input)
    }
}

#[cfg(feature = "bytemuck")]
include!("public_key_bytes/bytemuck.rs");

#[cfg(feature = "ed25519-dalek")]
include!("public_key_bytes/ed25519-dalek.rs");

#[cfg(feature = "eloquent")]
include!("public_key_bytes/eloquent.rs");

#[cfg(feature = "iroh")]
include!("public_key_bytes/iroh.rs");

#[cfg(feature = "libsql")]
include!("public_key_bytes/libsql.rs");

#[cfg(feature = "rocket")]
include!("public_key_bytes/rocket.rs");

#[cfg(feature = "turso")]
include!("public_key_bytes/turso.rs");
