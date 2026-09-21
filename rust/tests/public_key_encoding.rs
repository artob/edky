// This is free and unencumbered software released into the public domain.

use edky::PublicKeyEncoding;

#[test]
fn enabled_encoding_names_parse_case_insensitively() {
    for &encoding in PublicKeyEncoding::VARIANTS {
        let name = encoding.to_string();
        let mixed_case: String = name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect();
        for input in [
            name.clone(),
            name.to_lowercase(),
            name.to_uppercase(),
            mixed_case,
        ] {
            assert_eq!(input.parse::<PublicKeyEncoding>().unwrap(), encoding);
        }
    }
}

#[test]
fn parsing_respects_enabled_codec_features() {
    for (name, enabled) in [
        ("base16", true),
        ("asimov", cfg!(feature = "base58")),
        ("base32z", cfg!(feature = "base32z")),
        ("base58", cfg!(feature = "base58")),
        ("base64", cfg!(feature = "base64")),
        ("base64url", cfg!(feature = "base64")),
        ("multibase", cfg!(feature = "multibase")),
        ("near", cfg!(feature = "base58")),
        ("openssh", cfg!(feature = "base64")),
    ] {
        assert_eq!(name.parse::<PublicKeyEncoding>().is_ok(), enabled, "{name}");
    }
}

#[test]
fn invalid_names_keep_the_existing_error_type_and_message() {
    for input in [
        "",
        "unknown",
        "hex",
        "iroh",
        "ipfs",
        "libp2p",
        "ssh-ed25519",
        " base16",
        "base16 ",
        "base16\n",
        "base16\0",
        "base-16",
        "báse16",
        "base１６",
    ] {
        let error: derive_more::FromStrError = input.parse::<PublicKeyEncoding>().unwrap_err();
        assert_eq!(
            error.to_string(),
            "Invalid `PublicKeyEncoding` string representation",
            "{input:?}"
        );
    }
}

#[cfg(feature = "clientele")]
#[test]
fn public_key_errors_convert_with_clientele_enabled() {
    let error: clientele::SysexitsError = edky::ParsePublicKeyError::InvalidPrefix.into();
    assert!(matches!(error, clientele::SysexitsError::EX_DATAERR));
}
