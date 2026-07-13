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
        use ParsePublicKeyError::*;
        use PublicKeyEncoding::*;
        let input = input.as_ref();
        let mut buffer = [0u8; 32];
        Ok(match encoding {
            Base16 => input.parse::<Self>()?,

            #[cfg(feature = "base58")]
            Asimov => {
                let Some(input) = input.strip_prefix("ⒶY") else {
                    return Err(InvalidPrefix);
                };
                bs58::decode(input).onto(&mut buffer)?;
                Self(buffer)
            },

            #[cfg(feature = "base58")]
            Base58 => {
                bs58::decode(input).onto(&mut buffer)?;
                Self(buffer)
            },

            #[cfg(feature = "base64")]
            Base64 => {
                data_encoding::BASE64.decode_mut(input.as_bytes(), &mut buffer)?;
                Self(buffer)
            },

            #[cfg(feature = "base64")]
            Base64Url => {
                data_encoding::BASE64URL_NOPAD.decode_mut(input.as_bytes(), &mut buffer)?;
                Self(buffer)
            },

            #[cfg(feature = "multibase")]
            Multibase => {
                // See: https://github.com/multiformats/multibase/blob/master/multibase.csv
                // See: https://github.com/multiformats/multicodec/blob/master/table.csv
                let Some(input) = input.strip_prefix("z") else {
                    return Err(InvalidPrefix);
                };
                let mut buffer = [0u8; 34];
                bs58::decode(input).onto(&mut buffer)?;
                if buffer[0] != 0xed {
                    return Err(InvalidPrefix);
                }
                if buffer[1] != 0x01 {
                    return Err(InvalidPrefix);
                }
                Self::from_slice(&buffer[2..])?
            },

            #[cfg(feature = "base58")]
            Near => {
                let Some(input) = input.strip_prefix("ed25519:") else {
                    return Err(InvalidPrefix);
                };
                bs58::decode(input).onto(&mut buffer)?;
                Self(buffer)
            },

            #[cfg(feature = "base64")]
            OpenSsh => {
                let input = input
                    .strip_prefix("ssh-ed25519 ")
                    .unwrap_or(input)
                    .as_bytes();

                const BUFFER_LEN: usize =
                    size_of::<u32>() + b"ssh-ed25519".len() + size_of::<u32>() + PUBLIC_KEY_LEN;
                let mut buffer = [0u8; BUFFER_LEN];

                let decode_len = data_encoding::BASE64.decode_len(input.len())?;
                if decode_len != buffer.len() {
                    return Err(InvalidLength(input.len()));
                }
                data_encoding::BASE64.decode_mut(input, &mut buffer[..decode_len])?;

                let input = buffer;
                let (slice, input) = input.split_at(size_of::<u32>());
                if !slice.eq(&(b"ssh-ed25519".len() as u32).to_be_bytes()) {
                    return Err(InvalidChars);
                };

                let (slice, input) = input.split_at(b"ssh-ed25519".len());
                if !slice.eq(b"ssh-ed25519") {
                    return Err(InvalidChars);
                };

                let (slice, input) = input.split_at(size_of::<u32>());
                if !slice.eq(&(PUBLIC_KEY_LEN as u32).to_be_bytes()) {
                    return Err(InvalidChars);
                };

                Self::from_slice(&input)?
            },

            #[cfg(feature = "base32z")]
            Base32z => {
                data_encoding_macro::new_encoding! {
                    symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
                }
                .decode_mut(input.as_bytes(), &mut buffer)?;
                Self(buffer)
            },

            _ => todo!(), // TODO
        })
    }

    #[cfg(feature = "alloc")]
    pub fn encode(&self, encoding: PublicKeyEncoding) -> Option<alloc::string::String> {
        use PublicKeyEncoding::*;
        Some(match encoding {
            Base16 => alloc::string::ToString::to_string(self),

            #[cfg(feature = "base58")]
            Asimov => {
                alloc::format!("ⒶY{}", bs58::encode(self.0).into_string())
            },

            #[cfg(feature = "base58")]
            Base58 => bs58::encode(self.0).into_string(),

            #[cfg(feature = "base64")]
            Base64 => data_encoding::BASE64.encode(self.as_bytes()),

            #[cfg(feature = "base64")]
            Base64Url => data_encoding::BASE64URL_NOPAD.encode(self.as_bytes()),

            #[cfg(feature = "multibase")]
            Multibase => {
                // See: https://github.com/multiformats/multibase/blob/master/multibase.csv
                // See: https://github.com/multiformats/multicodec/blob/master/table.csv
                let mut buffer = [0u8; 34];
                buffer[0] = 0xed;
                buffer[1] = 0x01;
                buffer[2..].copy_from_slice(&self.0);
                alloc::format!("z{}", bs58::encode(buffer).into_string())
            },

            #[cfg(feature = "base58")]
            Near => {
                alloc::format!("ed25519:{}", bs58::encode(self.0).into_string())
            },

            #[cfg(feature = "base64")]
            OpenSsh => {
                let mut payload = alloc::vec::Vec::new();
                payload.extend_from_slice(&(b"ssh-ed25519".len() as u32).to_be_bytes());
                payload.extend_from_slice(b"ssh-ed25519");
                payload.extend_from_slice(&(PUBLIC_KEY_LEN as u32).to_be_bytes());
                payload.extend_from_slice(self.as_bytes());
                alloc::format!("ssh-ed25519 {}", data_encoding::BASE64.encode(&payload))
            },

            #[cfg(feature = "base32z")]
            Base32z => data_encoding_macro::new_encoding! {
                symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
            }
            .encode(self.as_bytes()),

            _ => return None, // not supported
        })
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

#[cfg(feature = "alloc")]
impl From<&alloc::vec::Vec<u8>> for PublicKeyBytes {
    fn from(input: &alloc::vec::Vec<u8>) -> Self {
        let mut bytes = [0u8; PUBLIC_KEY_LEN];
        let len = bytes.len().min(input.len());
        bytes[..len].copy_from_slice(&input[..len]);
        Self(bytes)
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
