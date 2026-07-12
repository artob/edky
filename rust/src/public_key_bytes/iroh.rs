// This is free and unencumbered software released into the public domain.

impl From<iroh::PublicKey> for PublicKeyBytes {
    fn from(input: iroh::PublicKey) -> Self {
        Self(input.as_bytes().clone())
    }
}

impl From<PublicKeyBytes> for iroh::PublicKey {
    fn from(input: PublicKeyBytes) -> Self {
        iroh::PublicKey::from_bytes(&input.into_bytes()).unwrap() // TODO
    }
}

impl From<iroh::EndpointAddr> for PublicKeyBytes {
    fn from(input: iroh::EndpointAddr) -> Self {
        Self(input.id.as_bytes().clone())
    }
}

impl From<PublicKeyBytes> for iroh::EndpointAddr {
    fn from(input: PublicKeyBytes) -> Self {
        let endpoint_id = iroh::EndpointId::from(input);
        iroh::EndpointAddr::from(endpoint_id)
    }
}
