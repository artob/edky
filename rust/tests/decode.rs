// This is free and unencumbered software released into the public domain.

use edky::{
    PUBLIC_KEY_FORMATS, PUBLIC_KEY_LEN, ParsePublicKeyError, PublicKeyBytes, PublicKeyEncoding,
};

/// Encodes arbitrary payload lengths independently of Edky's 32-byte key type.
fn encode_payload(encoding: PublicKeyEncoding, payload: &[u8]) -> String {
    use PublicKeyEncoding::*;
    match encoding {
        Base16 => payload.iter().map(|byte| format!("{byte:02x}")).collect(),
        #[cfg(feature = "base58")]
        Asimov => format!("ⒶY{}", base58(payload)),
        #[cfg(feature = "base58")]
        Base58 => base58(payload),
        #[cfg(feature = "base58")]
        Near => format!("ed25519:{}", base58(payload)),
        #[cfg(feature = "multibase")]
        Multibase => {
            let mut framed = vec![0xed, 0x01];
            framed.extend_from_slice(payload);
            format!("z{}", base58(&framed))
        },
        #[cfg(feature = "base64")]
        Base64 => data_encoding::BASE64.encode(payload),
        #[cfg(feature = "base64")]
        Base64Url => data_encoding::BASE64URL_NOPAD.encode(payload),
        #[cfg(feature = "base64")]
        OpenSsh => {
            // Keep the declared key length at 32 to expose truncated wire payloads.
            let mut framed = b"\0\0\0\x0bssh-ed25519\0\0\0\x20".to_vec();
            framed.extend_from_slice(payload);
            format!("ssh-ed25519 {}", data_encoding::BASE64.encode(&framed))
        },
        #[cfg(feature = "base32z")]
        Base32z => data_encoding_macro::new_encoding! {
            symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
        }
        .encode(payload),
        _ => panic!("missing test encoder for {encoding:?}"),
    }
}

#[cfg(feature = "base58")]
fn base58(payload: &[u8]) -> String {
    // Use the slice API so these tests don't require bs58's alloc feature.
    let mut output = vec![0; payload.len() * 2];
    let len = bs58::encode(payload).onto(output.as_mut_slice()).unwrap();
    output.truncate(len);
    String::from_utf8(output).unwrap()
}

#[test]
fn decoded_payloads_must_be_exactly_32_bytes() {
    for &encoding in PublicKeyEncoding::VARIANTS {
        for len in 0..=PUBLIC_KEY_LEN * 2 + 1 {
            for byte in [0x00, 0x42, 0xff] {
                let payload = vec![byte; len];
                let input = encode_payload(encoding, &payload);
                let result = PublicKeyBytes::decode(encoding, &input);
                if len == PUBLIC_KEY_LEN {
                    assert_eq!(result.unwrap().as_slice(), payload, "{encoding:?}: {input}");
                } else {
                    assert!(
                        matches!(result, Err(ParsePublicKeyError::InvalidLength(_))),
                        "{encoding:?}, {len} bytes: {result:?} for {input:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn malformed_inputs_return_errors_without_panicking() {
    for &encoding in PublicKeyEncoding::VARIANTS {
        for input in [
            "",
            "A",
            "AAAA",
            "AA",
            "yy",
            "é",
            "ⒶY",
            "ed25519:",
            "zK36",
            "ssh-ed25519 ",
        ] {
            assert!(
                PublicKeyBytes::decode(encoding, input).is_err(),
                "{encoding:?}: {input:?}"
            );
        }

        let valid = encode_payload(encoding, &[0x42; PUBLIC_KEY_LEN]);
        for invalid_char in ['!', '\0', 'é'] {
            let mut input = valid.clone();
            input.pop();
            input.push(invalid_char);
            assert!(
                PublicKeyBytes::decode(encoding, &input).is_err(),
                "{encoding:?}: {input:?}"
            );
        }
    }
}

#[test]
fn oversized_inputs_return_errors_without_panicking() {
    let payload = "A".repeat(4096);
    for &encoding in PublicKeyEncoding::VARIANTS {
        let prefix = PUBLIC_KEY_FORMATS
            .iter()
            .find(|format| format.encoding() == encoding && format.prefix().is_some())
            .and_then(|format| format.prefix())
            .unwrap_or("");
        let input = format!("{prefix}{payload}");
        assert!(
            PublicKeyBytes::decode(encoding, &input).is_err(),
            "{encoding:?}"
        );
    }
}

#[cfg(feature = "base64")]
#[test]
fn invalid_base64_padding_and_trailing_bits_are_rejected() {
    use PublicKeyEncoding::{Base64, Base64Url};
    for (encoding, input) in [
        (Base64, "A".repeat(43)),
        (Base64, format!("{}=AAA", "A".repeat(40))),
        (Base64, format!("{}A===", "A".repeat(40))),
        (Base64, format!("{}B=", "A".repeat(42))),
        (Base64Url, format!("{}B", "A".repeat(42))),
        (Base64Url, format!("{}=", "A".repeat(43))),
    ] {
        assert!(
            PublicKeyBytes::decode(encoding, &input).is_err(),
            "{encoding:?}: {input:?}"
        );
    }
}

#[cfg(feature = "base32z")]
#[test]
fn invalid_base32z_trailing_bits_are_rejected() {
    // A 32-byte key leaves four unused bits in the final Base32 symbol.
    let input = format!("{}b", "y".repeat(51));
    assert!(PublicKeyBytes::decode(PublicKeyEncoding::Base32z, input).is_err());
}

#[cfg(feature = "base64")]
#[test]
fn openssh_also_accepts_an_unprefixed_wire_payload() {
    let payload = [0x42; PUBLIC_KEY_LEN];
    let input = encode_payload(PublicKeyEncoding::OpenSsh, &payload);
    let input = input.strip_prefix("ssh-ed25519 ").unwrap();
    assert_eq!(
        PublicKeyBytes::decode(PublicKeyEncoding::OpenSsh, input).unwrap(),
        PublicKeyBytes::from(payload)
    );
}
