# Gate Results

Date: 2026-06-27

| Command | Result | Notes |
| --- | --- | --- |
| `.venv/bin/python tools/snowfreeze_observed/march_april_residual_attribution.py` | PASS | Regenerated JSON/Markdown attribution artifacts from the 10.3.8 coupled WAT report. |
| `cargo fmt --check` | PASS | Clean after formatting the new Rust guard test. |
| `cargo test --test snowdensity10_3_9_march_april_residual_attribution` | PASS | 3 tests passed. |
| `cargo clippy --test snowdensity10_3_9_march_april_residual_attribution -- -D warnings` | PASS | Focused clippy gate passed. |

## Boundary Scan

The package changed only diagnostic tooling, documentation, package artifacts,
and a focused guard test. It did not edit production snow/frost runtime code,
fixtures, public schemas, parser/runfile/user surfaces, default selectors,
coefficients, radiation, canopy, phase partition, density, melt, rain heat,
longwave, or frost physics.

## Execution Summary

- Candidate baseline: `coe_liquid_holding_capacity_v1`.
- Remaining paired failures after 10.3.8: `761/1415`.
- March/April failures: `282/463` paired March/April rows.
- March/April share of remaining failures: `282/761`.
- Recommended next process:
  `SPRING-PACK-DEPLETION-AND-COMPACTION-ADJUDICATION`.
