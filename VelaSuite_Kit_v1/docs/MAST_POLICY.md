# MastPolicy
Signed token (COSE/CBOR or PASETO) that activates features allowed by VelaKeys for a specific device (KeelID).

## Claims (canonical)
- `iss` (issuer) — `bitacora`
- `sub` (device DID) — KeelID
- `exp` — expiry (short-lived)
- `features` — array of `VELA.*` aliases
- `min_template_versions` — component→minVersion map
- `allowed_hashes` — allowlist of signed template hashes
- `offline_grace_s` — offline window in seconds
- `app_version` — accepted range (semver)
- `attestation_required` — boolean
- `audio_challenge` — optional VelaSonar params

See `spec/mastpolicy.schema.json` and `spec/examples/mastpolicy.example.json`.