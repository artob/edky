// This is free and unencumbered software released into the public domain.

use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PublicKeyEncoding {
    #[default]
    Hex,

    #[cfg(feature = "base58")]
    Asimov,

    #[cfg(feature = "base32z")]
    Base32z,

    #[cfg(feature = "base58")]
    Base58,

    #[cfg(feature = "base64")]
    Base64,

    #[cfg(feature = "base64")]
    Base64Url,

    /// See: <https://github.com/multiformats/multibase>
    #[cfg(feature = "multibase")]
    Multibase,

    #[cfg(feature = "base58")]
    Near,

    #[cfg(feature = "base64")]
    OpenSsh,
}

impl PublicKeyEncoding {
    pub const VARIANTS: &'static [Self] = &[
        Self::Hex,
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
