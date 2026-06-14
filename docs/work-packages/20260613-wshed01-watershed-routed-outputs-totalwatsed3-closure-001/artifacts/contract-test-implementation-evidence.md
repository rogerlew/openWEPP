# Contract Test Implementation Evidence

Status: W-A executed

Evidence mode: Static

No tests were authored in W-A because the increment forbids production edits.

Required W-B tests:

- Add zero-impoundment parser fixtures for explicit datver + `jpond=0`.
- Assert strict/compatibility success when `expected_structural_count=Some(0)`.
- Assert typed mismatch when `expected_structural_count=Some(1)` and
  `jpond=0`.
- Preserve existing malformed-count and active-impoundment tests.
- Add a watershed CLI behavior test that confirms arboreal-dendrite-style
  no-impoundment input proceeds past the current `CLIWAT-E-010` seam.

Required W-C tests:

- Assert all 14 watershed outputs exist.
- Assert `totalwatsed3.parquet` is not the one-row default writer surface for a
  real routed run.
- Assert reported depth columns match volume-derived depths.
- Add an anti-placeholder gate for required water-balance operands.
