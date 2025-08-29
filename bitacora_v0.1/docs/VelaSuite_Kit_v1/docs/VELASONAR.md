# VelaSonar™ — Audio-FFT Challenge
**Purpose:** playful liveness/hardening. Not a primary factor.

## Flow
1) Policy embeds `audio_challenge` (seed, nonce, sr, dur_ms, bins, window).
2) Device synthesizes a short audio (chirp/noise), plays & records (mic path).
3) Compute FFT/STFT, quantize peaks → vector; build HMAC(seed||nonce||vector).
4) Sign + send; server reconstructs expected, tolerates jitter, checks anti-replay.

## Anti-Replay
- Nonce + short exp
- Randomize frequency bins & window per challenge
- Reject suspiciously perfect loopback latency

## Config example
```yaml
audio_challenge:
  enabled: true
  sr: 16000
  dur_ms: 800
  window: hann
  bins: 64
```