// This is free and unencumbered software released into the public domain.

use crate::{PUBLIC_KEY_LEN, ParsePublicKeyError, PublicKeyBytes, PublicKeyEncoding};

pub fn decode(
    encoding: PublicKeyEncoding,
    input: impl AsRef<str>,
) -> Result<PublicKeyBytes, ParsePublicKeyError> {
    use ParsePublicKeyError::*;
    use PublicKeyEncoding::*;
    let input = input.as_ref();
    let mut buffer = [0u8; 32];
    Ok(match encoding {
        Base16 => input.parse::<PublicKeyBytes>()?,

        #[cfg(feature = "base58")]
        Asimov => {
            let Some(input) = input.strip_prefix("ⒶY") else {
                return Err(InvalidPrefix);
            };
            bs58::decode(input).onto(&mut buffer)?;
            PublicKeyBytes(buffer)
        },

        #[cfg(feature = "base58")]
        Base58 => {
            bs58::decode(input).onto(&mut buffer)?;
            PublicKeyBytes(buffer)
        },

        #[cfg(feature = "base64")]
        Base64 => {
            data_encoding::BASE64.decode_mut(input.as_bytes(), &mut buffer)?;
            PublicKeyBytes(buffer)
        },

        #[cfg(feature = "base64")]
        Base64Url => {
            data_encoding::BASE64URL_NOPAD.decode_mut(input.as_bytes(), &mut buffer)?;
            PublicKeyBytes(buffer)
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
            PublicKeyBytes::from_slice(&buffer[2..])?
        },

        #[cfg(feature = "base58")]
        Near => {
            let Some(input) = input.strip_prefix("ed25519:") else {
                return Err(InvalidPrefix);
            };
            bs58::decode(input).onto(&mut buffer)?;
            PublicKeyBytes(buffer)
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

            PublicKeyBytes::from_slice(&input)?
        },

        #[cfg(feature = "base32z")]
        Base32z => {
            data_encoding_macro::new_encoding! {
                symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
            }
            .decode_mut(input.as_bytes(), &mut buffer)?;
            PublicKeyBytes(buffer)
        },

        _ => todo!(), // TODO
    })
}
