// This is free and unencumbered software released into the public domain.

use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PublicKeyEncoding {
    #[cfg(feature = "base58")]
    Asimov,

    #[cfg(feature = "base58")]
    Base58,

    #[cfg(feature = "base64")]
    Base64,

    #[cfg(feature = "base64")]
    Base64Url,

    #[default]
    Hex,

    #[cfg(feature = "base58")]
    Near,

    #[cfg(feature = "base64")]
    OpenSsh,

    #[cfg(feature = "z32")]
    Z32,
}
