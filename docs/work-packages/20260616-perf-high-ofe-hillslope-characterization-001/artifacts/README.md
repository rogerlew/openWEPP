# Artifacts

Status: complete (Codex executed 2026-06-16).

Evidence and verdict artifacts for PERFHO01 (high-OFE hillslope performance
characterization). Execution outputs:

- `perf-profile-evidence.md` - profiler output + wall-clock % breakdown.
- `perf-scaling-curve.md` - wall-clock per sim-day vs OFE count (1-5 -> 19 OFEs).
- `perfho01-verdict.md` - cost attribution, scaling exponent, and the
  acceptable-as-is / named-optimization-follow-on recommendation (with the
  bit-identity / determinism bound any future fix must hold).
- `runfiles/` - package-local runfiles used for `/tmp/perfho01` measurements.
