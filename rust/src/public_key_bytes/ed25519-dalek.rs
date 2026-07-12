// This is free and unencumbered software released into the public domain.

impl From<ed25519_dalek::VerifyingKey> for PublicKeyBytes {
    fn from(input: ed25519_dalek::VerifyingKey) -> Self {
        Self(input.to_bytes())
    }
}
