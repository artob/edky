// This is free and unencumbered software released into the public domain.

use crate::{PUBLIC_KEY_LEN, ParsePublicKeyError, PublicKeyBytes, PublicKeyEncoding};

/// Decodes a public key using the selected encoding and fixed-size buffers.
///
/// The decoded key must contain exactly [`PUBLIC_KEY_LEN`] bytes, excluding any
/// format framing. Short keys are not zero-padded and long keys are not truncated.
/// This validates the representation, not whether the bytes describe a valid
/// Ed25519 point. Available encodings depend on [`PublicKeyEncoding`]'s features.
/// OpenSSH accepts either a `ssh-ed25519 ` prefix followed by the Base64 wire
/// payload, or the bare wire payload.
///
/// # Errors
///
/// Returns [`ParsePublicKeyError`] for invalid lengths, encoding characters,
/// padding, prefixes, or framing. Malformed input is rejected without panicking.
///
/// # Examples
///
/// ```
/// use edky::{decode, PublicKeyBytes, PublicKeyEncoding};
///
/// let key = decode(
///     PublicKeyEncoding::Base16,
///     "0000000000000000000000000000000000000000000000000000000000000000",
/// )?;
/// assert_eq!(key, PublicKeyBytes::ZERO);
/// # Ok::<(), edky::ParsePublicKeyError>(())
/// ```
pub fn decode(
    encoding: PublicKeyEncoding,
    input: impl AsRef<str>,
) -> Result<PublicKeyBytes, ParsePublicKeyError> {
    use ParsePublicKeyError::*;
    use PublicKeyEncoding::*;
    let input = input.as_ref();
    let mut buffer = [0u8; PUBLIC_KEY_LEN];
    Ok(match encoding {
        Base16 => input.parse::<PublicKeyBytes>()?,

        #[cfg(feature = "base58")]
        Asimov => {
            let Some(input) = input.strip_prefix("ⒶY") else {
                return Err(InvalidPrefix);
            };
            let len = bs58::decode(input).onto(&mut buffer)?;
            PublicKeyBytes::from_slice(&buffer[..len])?
        },

        #[cfg(feature = "base58")]
        Base58 => {
            let len = bs58::decode(input).onto(&mut buffer)?;
            PublicKeyBytes::from_slice(&buffer[..len])?
        },

        #[cfg(feature = "base64")]
        Base64 => {
            // Padded Base64's output-size estimate includes the padding byte.
            let mut buffer = [0u8; PUBLIC_KEY_LEN + 1];
            let len = decode_data_encoding(&data_encoding::BASE64, input.as_bytes(), &mut buffer)?;
            PublicKeyBytes::from_slice(&buffer[..len])?
        },

        #[cfg(feature = "base64")]
        Base64Url => {
            let len = decode_data_encoding(
                &data_encoding::BASE64URL_NOPAD,
                input.as_bytes(),
                &mut buffer,
            )?;
            PublicKeyBytes::from_slice(&buffer[..len])?
        },

        #[cfg(feature = "multibase")]
        Multibase => {
            // See: https://github.com/multiformats/multibase/blob/master/multibase.csv
            // See: https://github.com/multiformats/multicodec/blob/master/table.csv
            let Some(input) = input.strip_prefix("z") else {
                return Err(InvalidPrefix);
            };
            let mut buffer = [0u8; 2 + PUBLIC_KEY_LEN];
            let len = bs58::decode(input).onto(&mut buffer)?;
            if len != buffer.len() {
                return Err(InvalidLength(len));
            }
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
            let len = bs58::decode(input).onto(&mut buffer)?;
            PublicKeyBytes::from_slice(&buffer[..len])?
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

            let len = decode_data_encoding(&data_encoding::BASE64, input, &mut buffer)?;
            if len != buffer.len() {
                return Err(InvalidLength(len));
            }

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

            PublicKeyBytes::from_slice(input)?
        },

        #[cfg(feature = "base32z")]
        Base32z => {
            let encoding = data_encoding_macro::new_encoding! {
                symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
            };
            let len = decode_data_encoding(&encoding, input.as_bytes(), &mut buffer)?;
            PublicKeyBytes::from_slice(&buffer[..len])?
        },
    })
}

#[cfg(any(feature = "base32z", feature = "base64"))]
fn decode_data_encoding(
    encoding: &data_encoding::Encoding,
    input: &[u8],
    buffer: &mut [u8],
) -> Result<usize, ParsePublicKeyError> {
    let decode_len = encoding.decode_len(input.len())?;
    if decode_len > buffer.len() {
        return Err(ParsePublicKeyError::InvalidLength(input.len()));
    }
    // decode_mut requires exactly decode_len output bytes, even for short input.
    Ok(encoding.decode_mut(input, &mut buffer[..decode_len])?)
}
