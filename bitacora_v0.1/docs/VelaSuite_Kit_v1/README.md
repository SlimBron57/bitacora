# VelaSuite Kit v1
**Components:** VelaKeys® (licensing), VelaLock™ (authN/authZ + vault), VelaSonar™ (audio FFT challenge), MastPolicy (signed policy), Bitácora Navigator mapping.

**Status:** Draft / MVP • 2025-08-29 01:26 UTC

## Modules
- **VelaKeys®** — product keys inspired by sails (VELA.* SKUs) mapped to Navigator features and specialists.
- **VelaLock™** — privacy-by-design security layer: device enclave + OPAQUE + signed MastPolicy + local encrypted templates.
- **VelaSonar™** — optional audio-FFT challenge for playful liveness/hardening (not a primary factor).
- **MastPolicy** — COSE/CBOR (or PASETO v4) signed token that “hoists” features permitted by VelaKeys.

See `docs/` and `spec/` for details, and `sdk/` for Rust skeletons.