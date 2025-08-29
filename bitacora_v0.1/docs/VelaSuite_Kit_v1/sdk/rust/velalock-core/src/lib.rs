//! VelaLock™ core crate (skeleton).
//! Provides enrollment, policy verification (MastPolicy), local vault, attestation hooks,
//! and VelaSonar™ challenge plumbing.

pub mod enroll;
pub mod policy;
pub mod vault;
pub mod attest;
pub mod sonar;
pub mod errors;
pub mod crypto;