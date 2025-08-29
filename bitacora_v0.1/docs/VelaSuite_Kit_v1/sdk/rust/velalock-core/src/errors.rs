use thiserror::Error;

#[derive(Error, Debug)]
pub enum VelaError {
    #[error("attestation failed")]
    Attestation,
    #[error("policy invalid: {0}")]
    PolicyInvalid(String),
    #[error("crypto error")]
    Crypto,
    #[error("vault error: {0}")]
    Vault(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}