// This is free and unencumbered software released into the public domain.

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for PublicKeyBytes {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for PublicKeyBytes {}
