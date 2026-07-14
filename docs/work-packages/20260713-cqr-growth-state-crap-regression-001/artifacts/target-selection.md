# Target Selection And Baseline

Evidence mode: Static + Ran.

## Source

- Repository HEAD/frozen base: `3071849a0aec2abf8c17fe2405ce468f1533f631`
- Target file SHA-256 before extraction:
  `4dc15e436b27fc650ee064e7847123735e5f4855ba23cc6deb54e7ec21b95064`
- Target file length before extraction: `1,653` lines.
- Fresh report:
  `/tmp/openwepp-acrap-live-20260713/workspace-crap.json`
- Fresh LCOV:
  `/tmp/openwepp-acrap-live-20260713/workspace.lcov`

## Exact Raw Row

| File | Function | Line | CC | Coverage | CRAP | Classification |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` | `DirectGrowthInputs::compute_equation_growth_state` | 659 | 31 | 97.43589743589743% | 31.01620054282776 | `E-PRODUCTION` |

The full adjudicated census was `3` raw, `2` exact historical
`R-OBSERVABILITY` adjudications, and `1` actionable. The target is ordinary
hand-authored scientific control flow and is not eligible for exclusion.

## Regression Cause

The terminal CQR campaign measured this function at CC 29 and CRAP
`29.01616`. The later SC-PLANT-001 `INV-PLANT-027` defect correction added the
required perennial cap-before-increment branch. That branch is correct but
raised CC to 31. The appropriate disposition is cohesive decomposition, not a
waiver or a coverage-only response.
