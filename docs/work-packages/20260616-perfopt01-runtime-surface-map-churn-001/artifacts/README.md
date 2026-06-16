# Artifacts

Status: queued (Codex to execute).

Evidence and disposition artifacts for PERFOPT01 (runtime-surface map-churn &
writeback-validation optimization). Expected on execution:

- `perfopt01-before-after-profiling-evidence.md` — wall-clock + profiler re-check
  (the named hot path shrank).
- `perfopt01-bit-identity-and-determinism-evidence.md` — `anchor_mismatches = 0`
  vs the M1 baseline (HBP byte-identity + parquet row/value-identity) and the
  pinned-seed reproducibility check.
- `perfopt01-line-count-governance-checklist.md`, `perfopt01-gate-results.md`.
- `perfopt01_disposition.md`, `perfopt01-worker-handoff.md` (naming `PERFHO02` if
  a residual gap remains), + dual review/verification artifacts.

Primary correctness gate: **bit-identical outputs** vs the pre-optimization
baseline — non-waivable. Determinism per `docs/numerics/`.
