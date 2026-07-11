// This is free and unencumbered software released into the public domain.

use core::{fmt::Display, ops::Deref};

pub const PUBLIC_KEY_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyBytes(pub(crate) [u8; PUBLIC_KEY_LEN]);

impl Display for PublicKeyBytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buffer = [0u8; PUBLIC_KEY_LEN * 2];
        const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
        for (i, &byte) in self.0.iter().enumerate() {
            buffer[i * 2] = HEX_CHARS[(byte >> 4) as usize];
            buffer[i * 2 + 1] = HEX_CHARS[(byte & 0x0f) as usize];
        }
        // Safe to unwrap as it's an ASCII buffer containing only HEX_CHARS:
        f.write_str(core::str::from_utf8(&buffer).unwrap())
    }
}

impl PublicKeyBytes {
    pub const ZERO: Self = Self([0u8; PUBLIC_KEY_LEN]);

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

#[cfg(feature = "alloc")]
impl From<&alloc::vec::Vec<u8>> for PublicKeyBytes {
    fn from(input: &alloc::vec::Vec<u8>) -> Self {
        let mut bytes = [0u8; PUBLIC_KEY_LEN];
        let len = bytes.len().min(input.len());
        bytes[..len].copy_from_slice(&input[..len]);
        Self(bytes)
    }
}

include!("public_key_bytes/ed25519-dalek.rs");
include!("public_key_bytes/iroh.rs");
