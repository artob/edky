// This is free and unencumbered software released into the public domain.

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for PublicKey {
    type Error = KeyError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}
