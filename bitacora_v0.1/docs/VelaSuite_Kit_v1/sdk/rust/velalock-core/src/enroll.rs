use crate::crypto::{Dek, Kek, derive_kek_from_opaque};
use crate::errors::VelaError;

#[derive(Clone, Debug)]
pub struct DeviceCtx {
    pub did: String,
    pub wrap_dek_b64: String,
    pub attestation_doc: Vec<u8>,
}

pub fn enroll_device(pass_or_gesture: &[u8]) -> Result<DeviceCtx, VelaError> {
    // 1) generate enclave keypair + DID (placeholder DID here)
    let did = format!("did:keel:{}", hex::encode(&rand_core::OsRng.next_u64().to_le_bytes()));
    // 2) create DEK + KEK
    let dek = Dek(vec![0u8;32]);
    let opaque_export = pass_or_gesture; // placeholder for PAKE export
    let kek = derive_kek_from_opaque(opaque_export);
    // 3) wrap DEK with KEK (placeholder XOR for demo only!)
    let wrap = dek.0.iter().zip(kek.0.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
    let wrap_b64 = base64::encode(wrap);
    // 4) fake attestation doc
    let attestation_doc = b"attn".to_vec();
    Ok(DeviceCtx{ did, wrap_dek_b64: wrap_b64, attestation_doc })
}