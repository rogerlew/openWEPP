# Runner Intake Lane Setup Mechanical Split

Status: complete.

Package type: Mechanical refactor ExecPlan.

Objective: split the execution/output/manifest tail out of
`crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
without changing runtime behavior, public API shape, formulas, thresholds,
guards, manifest schema semantics, or output bytes.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for mechanical-move review,
public API parity review, line-count governance review, and gate-evidence
audit. Expected outputs are compact Markdown findings summarized into
`artifacts/review-disposition.md`; subagents may not edit files.

## Rationale

R6J closed direct publication cutover but left
`00_runner_intake_and_lane_setup.rs` at `2997` lines, just below the 3000-line
hard threshold. The file mixes intake/static setup, climate-day execution,
output writing, manifest assembly, and run entrypoints. The first safe seam is
the execution/output/manifest tail because it is already clustered after static
runtime setup and can remain in the same Rust module through `include!`.

## Scope

In scope:

- Move `execute_hillslope_climate_days` and the subsequent execution,
  provenance, output, manifest, run-entrypoint, and direct-runtime-skeleton
  helper block into a new included section file.
- Add that included section to `crates/openwepp-runner/src/hillslope/mod.rs`
  before tests.
- Preserve all item names, signatures, visibility, call sites, and behavior.
- Update package artifacts and work-package catalog.

Out of scope:

- Formula, unit, threshold, guard, schema, manifest meaning, or output behavior
  changes.
- Public API changes.
- Opportunistic cleanup beyond compile/format fallout from the move.
- Additional module-system migration away from the existing `include!` pattern.

## Write Set

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/mofe01_per_ofe_state_contract.rs`
- `tests/integration/mofe01_inter_ofe_route_contract.rs`
- `docs/work-packages/20260622-runner-intake-lane-setup-mechanical-split-001/**`
- `docs/work-packages/README.md`

## Plan

1. Capture pre-refactor symbol and line-count evidence.
2. Mechanically move the declared tail block into
   `05_runner_execution_and_outputs.rs`.
3. Include the new file from `hillslope/mod.rs` before tests.
4. Run focused compile/tests and fix only mechanical fallout.
5. Run full closure gates:
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
6. Run `git diff --check` and scoped Markdown lint.
7. Complete artifacts: modularization plan, API parity, line-count governance,
   review disposition, verification, and handoff.

## Acceptance Criteria

- `00_runner_intake_and_lane_setup.rs` is below 2000 lines.
- New included section file is below 2000 lines.
- Public exported runner surface remains unchanged:
  `execute_hillslope_run` and `execute_hillslope_run_with_runtime_selection`
  continue to be re-exported from `crates/openwepp-runner/src/lib.rs`.
- No intended semantic diff exists beyond item relocation and include wiring.
- Required closure gates pass and are recorded.
- No `.rs` file touched by this package remains at or above 3000 lines.

## Progress

- [x] Scaffold package.
- [x] Capture pre-refactor evidence.
- [x] Move execution/output/manifest tail.
- [x] Validate focused checks.
- [x] Validate full closure gates.
- [x] Complete review, verification, line-count, and handoff artifacts.

## Outcomes

Final disposition: `COMPLETE-RUNNER-INTAKE-LANE-SETUP-MECHANICAL-SPLIT`.

The execution/output/manifest and public runner-entrypoint tail was moved from
`00_runner_intake_and_lane_setup.rs` into
`05_runner_execution_and_outputs.rs` as a byte-identical relocation. The
original runner setup file is now `1741` lines and the new execution/output
section is `1255` lines. Static source-level MOFE tests were updated to scan
both included runner files after their asserted tokens moved with the tail.

Closure gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`
- Scoped Markdown lint for the package and work-package catalog
