// This is free and unencumbered software released into the public domain.

use core::str::FromStr;
use derive_more::{Display, FromStrError};

/// A public-key encoding available under the enabled Cargo features.
///
/// [`FromStr`] accepts variant names with ASCII case-insensitive matching and
/// does not allocate. Unknown names and disabled encodings return
/// [`derive_more::FromStrError`]. Whitespace is not trimmed. Format aliases such
/// as `hex` and `iroh` are listed in [`crate::PUBLIC_KEY_FORMATS`] and resolved by
/// the CLI, rather than by this parser.
///
/// Base16 is always available, including without `alloc` or `std`. Other variants
/// are gated by their codec features. [`core::fmt::Display`] uses the variant's
/// spelling, such as `Base16`.
///
/// # Examples
///
/// ```
/// use edky::PublicKeyEncoding;
///
/// assert_eq!("BaSe16".parse(), Ok(PublicKeyEncoding::Base16));
/// assert!("unknown".parse::<PublicKeyEncoding>().is_err());
/// ```
#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl FromStr for PublicKeyEncoding {
    type Err = FromStrError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            input if input.eq_ignore_ascii_case("base16") => Self::Base16,
            #[cfg(feature = "base58")]
            input if input.eq_ignore_ascii_case("asimov") => Self::Asimov,
            #[cfg(feature = "base32z")]
            input if input.eq_ignore_ascii_case("base32z") => Self::Base32z,
            #[cfg(feature = "base58")]
            input if input.eq_ignore_ascii_case("base58") => Self::Base58,
            #[cfg(feature = "base64")]
            input if input.eq_ignore_ascii_case("base64") => Self::Base64,
            #[cfg(feature = "base64")]
            input if input.eq_ignore_ascii_case("base64url") => Self::Base64Url,
            #[cfg(feature = "multibase")]
            input if input.eq_ignore_ascii_case("multibase") => Self::Multibase,
            #[cfg(feature = "base58")]
            input if input.eq_ignore_ascii_case("near") => Self::Near,
            #[cfg(feature = "base64")]
            input if input.eq_ignore_ascii_case("openssh") => Self::OpenSsh,
            _ => return Err(FromStrError::new("PublicKeyEncoding")),
        })
    }
}

impl PublicKeyEncoding {
    /// All encoding variants available under the enabled Cargo features.
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
