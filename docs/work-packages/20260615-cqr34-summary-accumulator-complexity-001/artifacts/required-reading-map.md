# CQR34 Required Reading Map

## Required Reading Completed

- [DIRECT] `AGENTS.md` - repository governance, CQR sequence, validation gates,
  kernel/science authority, and truthfulness requirements.
- [DIRECT] `crates/AGENTS.md` - crate-level Rust expectations and validation
  posture.
- [DIRECT] `docs/work-packages/AGENTS.md` - package lifecycle, review,
  verification, and closure requirements.
- [DIRECT] `docs/work-packages/cqr-burndown-execplan.md` - CQR34 target row,
  ordering, package/push/tracker rules, and acceptance criteria.
- [DIRECT] `docs/standards/mechanical-refactor-authoring-guide.md` -
  behavior-preserving refactor constraints.
- [DIRECT] `docs/standards/code-quality-refactor-authoring-guide.md` -
  characterization, before/after metrics, and closure evidence requirements.
- [DIRECT] `docs/decisions/0021-module-coverage-closure-thresholds.md` -
  module coverage thresholds and warning posture.
- [DIRECT] `docs/specifications/science-contracts/AGENTS.md` - kernel-facing
  science authority and no-surrogate behavior requirements.

## On-Demand Reading

- [DIRECT] `crates/openwepp-summary-accumulator/src/lib.rs` - target enum,
  display implementation, accumulator tests, rollup behavior, output symbols,
  and error/source behavior.
- [DIRECT] `crates/openwepp-sim-contract/src/status.rs` - `StatusError`
  construction/display semantics for characterization.
- [DIRECT] `crates/openwepp-comparator-metadata/src/lib.rs` -
  `ComparatorTierRoutingError` construction/display semantics for
  characterization.

## Deferred Reading

No broader kernel process contracts were needed because the package scope is a
private display-helper decomposition and focused characterization only. The
science-contract guard remains active: stop and hold if the work requires
changing runtime publication behavior, formulas, units, aliases, symbols,
typed guards, parser compatibility, or science authority.
