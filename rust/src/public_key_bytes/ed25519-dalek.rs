// This is free and unencumbered software released into the public domain.

#[cfg(feature = "ed25519-dalek")]
impl From<ed25519_dalek::VerifyingKey> for PublicKeyBytes {
    fn from(input: ed25519_dalek::VerifyingKey) -> Self {
        Self(input.to_bytes())
    }
}
