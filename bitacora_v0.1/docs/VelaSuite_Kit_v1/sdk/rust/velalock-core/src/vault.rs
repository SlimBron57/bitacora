use crate::crypto::{Dek, Kek};
use crate::errors::VelaError;

pub struct Vault;

impl Vault {
    pub fn open(_wrap_b64: &str, _kek: &Kek) -> Result<Dek, VelaError> {
        // TODO: unwrap via AEAD; placeholder
        Ok(Dek(vec![0u8;32]))
    }
}