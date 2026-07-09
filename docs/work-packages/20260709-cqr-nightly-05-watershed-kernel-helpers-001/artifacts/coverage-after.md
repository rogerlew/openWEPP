# Coverage After

Evidence label: Static/Ran.

Status: `COMPLETE`

Focused after commands:

- `cargo llvm-cov -p openwepp-watershed-orchestrator --lcov --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused.lcov`
  - exit `0`.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --json --summary-only --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused-summary.json`
  - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`

Eligible production surface:

- Production code ends before the root-level test-only block at line `589`.
- Production-only LCOV filter: `DA` records with source line `<= 587`.
- Test-only helper and `#[test]` functions are excluded from closure metrics.

Production-only LCOV line coverage after:

- `LF:532`
- `LH:487`
- Line coverage: `91.54135338345864%`

Whole-file LCOV line coverage after, including inline tests:

- `LF:856`
- `LH:810`
- Line coverage: `94.62616822429906%`

Production-only full JSON region coverage after:

- Production regions: `517 / 558`, `92.65232974910394%`
- Weakest production function region floor:
  `integrate_impoundment_stage_with_adaptive_retry`, `79 / 94`,
  `84.04255319148936%`

Whole-file JSON summary after, including inline tests:

- Lines: `811 / 857`, `94.63243873978999%`
- Regions: `922 / 964`, `95.64315352697096%`
- Functions: `39 / 39`, `100.0%`
- Instantiations: `40 / 40`, `100.0%`

Delta from baseline LCOV:

- Baseline: `LF:484`, `LH:262`, `54.13223140495868%`
- After production-only: `LF:532`, `LH:487`, `91.54135338345864%`
- Production-line coverage delta: `+37.40912197849996` percentage points.
