# Security & Threat Model (summary)
**Threats:** device theft, cloning, MITM, server compromise, on-chain scraping, template tampering, device loss.

**Mitigations:** non-exportable enclave keys, OPAQUE-derived KEK, signed MastPolicy, template signatures + hash allowlist, attestation, short policy TTL + offline grace, Shamir recovery 2-of-3, DP telemetry (opt-in).

**Biometrics/Gestures:** on-device only; never exfiltrate raw images/audio.