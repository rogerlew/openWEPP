# Implementation Log

Evidence mode: Static/Ran.

## Stage 0 - Authority

Static:

- ADR-0031 is accepted in
  `docs/decisions/0031-delete-compatibility-runtime-single-authority-terminal.md`.
- Rollback reference before full deletion:
  `45852c25a36304ddd2e642f3b825f68f38464958`.

## Stage 1 - Runtime Selector

Ran:

- Removed `HillslopeRuntimeSelection::Compatibility` and the disabled default
  activation branch from `crates/openwepp-runner/src/api.rs`.
- Removed `--compatibility-runtime` from `openwepp-cli-hill` parsing and help.
- Updated snowbench and observed frost harness execution helpers to request only
  `--direct-production-executor`.
- Updated direct-only provenance tests to expect `direct-publication-frame`
  sources and real calendar-year WB13 row keys.

## Stage 2 - Scheduler/Day-Frame Runtime

Static/Ran:

- Attempted to remove the unreachable scheduler execution branch and old
  scheduler publication/output branches from
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- `cargo check -p openwepp-runner` then compiled but emitted 351 dead-code
  warnings from the scheduler lifecycle, WB13 scheduler publication, HPHYS trace,
  compatibility row builders, and legacy scheduler tests. This would fail the
  required `clippy -D warnings` gate.
- The attempted broad deletion was backed out to keep the worktree green. The
  remaining scheduler/day-frame code is unreachable from public runtime
  selection but still compiled and test-backed.

## Stage 3 - Carrier Types And Proof

HOLD:

- Static scan after Stage 1 found no executable public selector references to
  `HillslopeRuntimeSelection::Compatibility`,
  `HillslopeDefaultRuntimeActivation::Disabled`, or `--compatibility-runtime`
  under `crates/` and `tools/`.
- Static scan still found approximately 1100 references to symbol-map carrier
  names across `openwepp-kernel-contract`, `openwepp-hillslope-orchestrator`,
  and runner support/test modules.
- Full carrier deletion requires a coherent support-boundary deletion/replacement
  of scheduler tests and symbol-keyed I/O helpers, not a blind file removal.
