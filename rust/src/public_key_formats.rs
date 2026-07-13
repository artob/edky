// This is free and unencumbered software released into the public domain.

use super::{PublicKeyEncoding, PublicKeyFormat};

pub const PUBLIC_KEY_FORMATS: &[PublicKeyFormat] = &[
    #[cfg(feature = "base58")]
    ASIMOV,
    //#[cfg(feature = "base16")]
    BASE16,
    #[cfg(feature = "base32z")]
    BASE32Z,
    #[cfg(feature = "base58")]
    BASE58,
    #[cfg(feature = "base64")]
    BASE64,
    #[cfg(feature = "base64")]
    BASE64URL,
    //#[cfg(feature = "base16")]
    HEX,
    #[cfg(feature = "multibase")]
    IPFS,
    #[cfg(feature = "base32z")]
    IROH,
    #[cfg(feature = "multibase")]
    LIBP2P,
    #[cfg(feature = "multibase")]
    MULTIBASE,
    #[cfg(feature = "base58")]
    NEAR,
    #[cfg(feature = "base64")]
    OPENSSH,
];

#[cfg(feature = "base58")]
pub(crate) const ASIMOV: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "asimov",
    encoding: PublicKeyEncoding::Asimov,
    prefix: Some("ⒶY"),
};

//#[cfg(feature = "base16")]
pub(crate) const BASE16: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "base16",
    encoding: PublicKeyEncoding::Base16,
    prefix: None,
};

#[cfg(feature = "base32z")]
pub(crate) const BASE32Z: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "base32z",
    encoding: PublicKeyEncoding::Base32z,
    prefix: None,
};

#[cfg(feature = "base58")]
pub(crate) const BASE58: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "base58",
    encoding: PublicKeyEncoding::Base58,
    prefix: None,
};

#[cfg(feature = "base64")]
pub(crate) const BASE64: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "base64",
    encoding: PublicKeyEncoding::Base64,
    prefix: None,
};

#[cfg(feature = "base64")]
pub(crate) const BASE64URL: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "base64url",
    encoding: PublicKeyEncoding::Base64Url,
    prefix: None,
};

//#[cfg(feature = "base16")]
pub(crate) const HEX: PublicKeyFormat = PublicKeyFormat::Alias("hex", PublicKeyEncoding::Base16);

#[cfg(feature = "multibase")]
pub(crate) const IPFS: PublicKeyFormat =
    PublicKeyFormat::Alias("ipfs", PublicKeyEncoding::Multibase);

#[cfg(feature = "base32z")]
pub(crate) const IROH: PublicKeyFormat = PublicKeyFormat::Alias("iroh", PublicKeyEncoding::Base32z);

#[cfg(feature = "multibase")]
pub(crate) const LIBP2P: PublicKeyFormat =
    PublicKeyFormat::Alias("libp2p", PublicKeyEncoding::Multibase);

#[cfg(feature = "multibase")]
pub(crate) const MULTIBASE: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "multibase",
    encoding: PublicKeyEncoding::Multibase,
    prefix: Some("z"),
};

#[cfg(feature = "base58")]
pub(crate) const NEAR: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "near",
    encoding: PublicKeyEncoding::Near,
    prefix: Some("ed25519:"),
};

#[cfg(feature = "base64")]
pub(crate) const OPENSSH: PublicKeyFormat = PublicKeyFormat::Entry {
    name: "openssh",
    encoding: PublicKeyEncoding::OpenSsh,
    prefix: Some("ssh-ed25519 "),
};
