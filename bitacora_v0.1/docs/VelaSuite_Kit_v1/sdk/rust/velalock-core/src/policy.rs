use serde::Deserialize;
use crate::errors::VelaError;

#[derive(Deserialize, Debug)]
pub struct MastPolicy {
    pub iss: String,
    pub sub: String,
    pub exp: u64,
    pub features: Vec<String>,
    pub offline_grace_s: u64,
    pub min_template_versions: Option<serde_json::Value>,
    pub allowed_hashes: Option<Vec<String>>,
    pub app_version: Option<String>,
    pub attestation_required: Option<bool>,
    pub audio_challenge: Option<serde_json::Value>,
}

pub fn verify_policy(_raw_cose: &[u8], _device_did: &str) -> Result<MastPolicy, VelaError> {
    // TODO: decode COSE, verify signature, check sub == device_did, check exp
    Err(VelaError::PolicyInvalid("not implemented".into()))
}