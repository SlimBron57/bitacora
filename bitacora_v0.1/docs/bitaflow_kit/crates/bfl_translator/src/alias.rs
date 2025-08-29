use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias(pub String);

#[derive(Debug, Error)]
pub enum AliasError {
    #[error("invalid alias format: {0}")]
    Invalid(String),
}

pub fn validate_alias(s: &str) -> Result<Alias, AliasError> {
    let re = Regex::new(r"^BITA-(TPL|DOC|ADR|SNAP|MAP)-[A-Z0-9]+(?:_[A-Z0-9]+)*(?:-[A-Z0-9]+(?:_[A-Z0-9]+)*)?-v[0-9]+(?:\+[A-Z0-9]+)*$").unwrap();
    if re.is_match(s) {
        Ok(Alias(s.to_string()))
    } else {
        Err(AliasError::Invalid(s.to_string()))
    }
}
