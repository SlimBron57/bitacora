use velalock_core::sonar::{run_challenge, SonarParams};

fn main() {
    let params = SonarParams { sr: 16000, dur_ms: 800, bins: 64, window: "hann" };
    let proof = run_challenge(b"seed", b"nonce", &params);
    println!("sonar_proof_len={}", proof.len());
}