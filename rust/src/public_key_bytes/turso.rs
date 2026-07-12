// This is free and unencumbered software released into the public domain.

impl turso::IntoValue for PublicKeyBytes {
    fn into_value(self) -> turso::Result<turso::Value> {
        Ok(turso::Value::Blob(self.0.to_vec()))
    }
}
