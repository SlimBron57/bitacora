use crate::errors::VelaError;

pub fn verify_attestation(_doc: &[u8]) -> Result<(), VelaError> {
    // TODO: platform-specific attestation verification
    Ok(())
}