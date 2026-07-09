# Final Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

Package result:

- Complete for CQR Nightly target #6.

Target:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

Outcome:

- Baseline target functions above CRAP `30`: `7`.
- Final target functions above CRAP `30`: `0`.
- Final max target CRAP: `23.069544598035826`.
- Targeted line coverage: `1782 / 1888 = 94.385593220339%`.
- Targeted deduplicated source-span region coverage:
  `2123 / 2274 = 93.35971855760774%`.
- ADR-0021 science-tier line/region thresholds and per-function region floor
  pass.

Behavior disposition:

- Behavior-preserving helper extraction and package-local test split only.
- No science formulas, thresholds, typed guard classes, serialization formats,
  runtime-symbol names, or public output meanings were changed.

Gate disposition:

- Current final gate bundle in
  `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/`
  records PASS for format, scoped diff-check, doc lint, focused orchestrator
  tests, wshedw5 integration tests, focused clippy, workspace clippy, full
  workspace nextest, and cargo deny.

Review and verification:

- Dual review passed after accepted artifact fixes.
- Dual verification passed after accepted write-set and raw-gate bundle fixes.

Completion commit boundary:

- This artifact set is ready for the ExecPlan-required completion commit; no
  additional package closure work remains before committing.
