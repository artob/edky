// This is free and unencumbered software released into the public domain.

use super::PublicKeyEncoding;

pub const PUBLIC_KEY_ENCODINGS: &[(&str, PublicKeyEncoding, Option<&str>)] = &[
    ("hex", PublicKeyEncoding::Hex, None),
    #[cfg(feature = "base58")]
    ("asimov", PublicKeyEncoding::Asimov, Some("ⒶY")),
    #[cfg(feature = "base32z")]
    ("base32z", PublicKeyEncoding::Base32z, None),
    #[cfg(feature = "base58")]
    ("base58", PublicKeyEncoding::Base58, None),
    #[cfg(feature = "base64")]
    ("base64", PublicKeyEncoding::Base64, None),
    #[cfg(feature = "base64")]
    ("base64url", PublicKeyEncoding::Base64Url, None),
    #[cfg(feature = "multibase")]
    ("multibase", PublicKeyEncoding::Multibase, Some("z")),
    #[cfg(feature = "base58")]
    ("near", PublicKeyEncoding::Near, Some("ed25519:")),
    #[cfg(feature = "base64")]
    ("openssh", PublicKeyEncoding::OpenSsh, Some("ssh-ed25519 ")),
];
