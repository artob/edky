// This is free and unencumbered software released into the public domain.

use crate::{PUBLIC_KEY_LEN, ParsePublicKeyError, PublicKeyBytes, PublicKeyEncoding};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

pub fn encode(encoding: PublicKeyEncoding, input: impl Into<PublicKeyBytes>) -> Option<String> {
    use PublicKeyEncoding::*;
    let input = input.into();
    Some(match encoding {
        Base16 => input.to_string(),

        #[cfg(feature = "base58")]
        Asimov => {
            format!("ⒶY{}", bs58::encode(input.0).into_string())
        },

        #[cfg(feature = "base58")]
        Base58 => bs58::encode(input.0).into_string(),

        #[cfg(feature = "base64")]
        Base64 => data_encoding::BASE64.encode(input.as_bytes()),

        #[cfg(feature = "base64")]
        Base64Url => data_encoding::BASE64URL_NOPAD.encode(input.as_bytes()),

        #[cfg(feature = "multibase")]
        Multibase => {
            // See: https://github.com/multiformats/multibase/blob/master/multibase.csv
            // See: https://github.com/multiformats/multicodec/blob/master/table.csv
            let mut buffer = [0u8; 34];
            buffer[0] = 0xed;
            buffer[1] = 0x01;
            buffer[2..].copy_from_slice(&input.0);
            format!("z{}", bs58::encode(buffer).into_string())
        },

        #[cfg(feature = "base58")]
        Near => {
            format!("ed25519:{}", bs58::encode(input.0).into_string())
        },

        #[cfg(feature = "base64")]
        OpenSsh => {
            let mut payload = Vec::new();
            payload.extend_from_slice(&(b"ssh-ed25519".len() as u32).to_be_bytes());
            payload.extend_from_slice(b"ssh-ed25519");
            payload.extend_from_slice(&(PUBLIC_KEY_LEN as u32).to_be_bytes());
            payload.extend_from_slice(input.as_bytes());
            format!("ssh-ed25519 {}", data_encoding::BASE64.encode(&payload))
        },

        #[cfg(feature = "base32z")]
        Base32z => data_encoding_macro::new_encoding! {
            symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
        }
        .encode(input.as_bytes()),

        _ => return None, // not supported
    })
}
