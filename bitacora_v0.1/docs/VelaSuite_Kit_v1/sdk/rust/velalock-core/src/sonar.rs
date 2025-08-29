//! VelaSonar™ placeholder API surfaces (no DSP here).

pub struct SonarParams {
    pub sr: u32,
    pub dur_ms: u32,
    pub bins: u32,
    pub window: &'static str,
}

pub fn run_challenge(_seed: &[u8], _nonce: &[u8], _params: &SonarParams) -> Vec<u8> {
    // TODO: synth + record + FFT + quantize + HMAC
    vec![]
}