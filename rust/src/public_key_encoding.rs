// This is free and unencumbered software released into the public domain.

use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PublicKeyEncoding {
    /// Base16 (aka hexadecimal)
    #[default]
    //#[cfg(feature = "base16")]
    Base16,

    /// ASIMOV Protocol
    #[cfg(feature = "base58")]
    Asimov,

    /// Base32z (aka z-base-32, Z32)
    #[cfg(feature = "base32z")]
    Base32z,

    /// Base58 (aka base58-btc)
    #[cfg(feature = "base58")]
    Base58,

    /// Base64
    #[cfg(feature = "base64")]
    Base64,

    /// Base64Url
    #[cfg(feature = "base64")]
    Base64Url,

    /// Multibase
    ///
    /// See: <https://github.com/multiformats/multibase>
    #[cfg(feature = "multibase")]
    Multibase,

    /// NEAR Protocol
    #[cfg(feature = "base58")]
    Near,

    /// OpenSSH Ed25519
    #[cfg(feature = "base64")]
    OpenSsh,
}

impl PublicKeyEncoding {
    pub const VARIANTS: &'static [Self] = &[
        //#[cfg(feature = "base16")]
        Self::Base16,
        #[cfg(feature = "base58")]
        Self::Asimov,
        #[cfg(feature = "base32z")]
        Self::Base32z,
        #[cfg(feature = "base58")]
        Self::Base58,
        #[cfg(feature = "base64")]
        Self::Base64,
        #[cfg(feature = "base64")]
        Self::Base64Url,
        #[cfg(feature = "multibase")]
        Self::Multibase,
        #[cfg(feature = "base58")]
        Self::Near,
        #[cfg(feature = "base64")]
        Self::OpenSsh,
    ];
}
