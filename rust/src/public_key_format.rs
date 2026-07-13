// This is free and unencumbered software released into the public domain.

use super::PublicKeyEncoding;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PublicKeyFormat {
    Entry {
        name: &'static str,
        encoding: PublicKeyEncoding,
        prefix: Option<&'static str>,
    },
    Alias(&'static str, PublicKeyEncoding),
}

impl PublicKeyFormat {
    pub fn name(&self) -> &str {
        match self {
            PublicKeyFormat::Entry { name, .. } => name,
            PublicKeyFormat::Alias(name, ..) => name,
        }
    }

    pub fn encoding(&self) -> PublicKeyEncoding {
        match self {
            PublicKeyFormat::Entry { encoding, .. } => *encoding,
            PublicKeyFormat::Alias(_, encoding) => *encoding,
        }
    }

    pub fn prefix(&self) -> Option<&str> {
        match self {
            PublicKeyFormat::Entry { prefix, .. } => *prefix,
            PublicKeyFormat::Alias(_, _) => None,
        }
    }
}
