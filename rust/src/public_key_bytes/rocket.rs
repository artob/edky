// This is free and unencumbered software released into the public domain.

impl<'r> rocket::request::FromParam<'r> for PublicKeyBytes {
    type Error = ParsePublicKeyError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}
