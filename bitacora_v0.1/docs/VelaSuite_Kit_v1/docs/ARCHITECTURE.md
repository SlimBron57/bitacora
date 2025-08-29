# Architecture (VelaLock™)
Privacy-by-design: server never sees DEK/templates; it only validates licenses and emits **MastPolicy**. Device verifies, unlocks DEK in enclave, and runs everything locally.

## Crypto Primitives (recommended)
- Signatures: Ed25519
- ECDH: X25519
- At-rest AEAD: XChaCha20-Poly1305
- Session: Noise IK/NX (snow) or TLS 1.3 (rustls)
- PAKE: OPAQUE

## Platform Attestation
- iOS: App Attest, DeviceCheck
- Android: Play Integrity, StrongBox
- Desktop: TPM/Windows Hello
- Web: WebAuthn/Passkeys

## High-level Flow
1) On-chain (or off-chain + anchor) licenses for VelaKeys (ERC‑1155 / SBT optional).
2) Enroll device (enclave keys) → KeelID (DID) + wrap(DEK, KEK).
3) Backend checks license → emits **MastPolicy** (signed) with allowed features/hashes.
4) Device validates policy, unwraps DEK (OPAQUE + enclave), loads local templates.
5) Offline mode with grace; revocation via policy expiration/deny.