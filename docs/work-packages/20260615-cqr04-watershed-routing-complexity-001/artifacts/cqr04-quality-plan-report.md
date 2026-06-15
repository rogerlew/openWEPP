# CQR04 Quality Plan Report

Static: behavior-preserving private helper extraction only.

## Target

- File:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
- Primary metric: every eligible target-file function CRAP `<= 30`.
- Secondary metric: remove target-file `clippy::too_many_lines` suppressions.
- Protected behavior: WS10/WS11/WS20-WS24 routing formulas, constants,
  branch predicates, guard IDs, publication symbols, and public crate APIs.

## Execution Shape

1. Baseline focused routing tests and metric artifacts.
2. Private helper extraction inside `routing.rs`.
3. Focused post-refactor routing tests.
4. Workspace format, clippy, test, deny gates.
5. After LCOV and CRAP evidence.
6. Dual local review and verification artifacts.

## Scope Control

No science contracts, parser projections, runner orchestration, output writers,
dependencies, or module splits were changed. No tests were edited; existing
WS10/WS11 and workspace suites were used as the behavior safety net.
