// This is free and unencumbered software released into the public domain.

impl libsql::params::IntoValue for PublicKeyBytes {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        Ok(libsql::Value::Blob(self.0.to_vec()))
    }
}
