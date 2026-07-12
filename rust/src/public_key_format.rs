// This is free and unencumbered software released into the public domain.

use super::PublicKeyEncoding;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PublicKeyFormat {
    pub name: &'static str,
    pub encoding: PublicKeyEncoding,
    pub prefix: Option<&'static str>,
}
