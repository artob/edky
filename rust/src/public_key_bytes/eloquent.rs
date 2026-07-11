// This is free and unencumbered software released into the public domain.

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for PublicKeyBytes {
    fn to_sql(&self) -> Result<alloc::string::String, eloquent::error::EloquentError> {
        use alloc::string::ToString;
        const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
        let mut buffer = [0u8; 2 + PUBLIC_KEY_LEN * 2 + 1]; // "X'{hex}'"
        buffer[0] = b'X';
        buffer[1] = b'\'';
        for (i, &byte) in self.0.iter().enumerate() {
            buffer[i * 2 + 2] = HEX_CHARS[(byte >> 4) as usize];
            buffer[i * 2 + 3] = HEX_CHARS[(byte & 0x0f) as usize];
        }
        buffer[2 + PUBLIC_KEY_LEN * 2] = b'\'';
        // Safe to unwrap as it's an ASCII buffer containing only HEX_CHARS:
        Ok(core::str::from_utf8(&buffer).unwrap().to_string())
    }
}
