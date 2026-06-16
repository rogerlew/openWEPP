# CQR23 - EROD19 Route Segment Complexity Refactor

Status: complete-with-warnings pending package commit and push

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR23 target in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`,
originally tracked as rank 17 with snapshot CRAP `352`, CC `79`, and coverage
`65%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The EROD19 hydrology phase is kernel-affecting. CQR23 must reduce local
complexity without changing phase dispatch, typed status behavior, runtime
symbols, units, formulas, float expression order, closure behavior, or
science-contract semantics.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target identity, line counts,
  suppression census, public API parity, behavior equivalence, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused characterization before production refactor when needed.
- Private behavior-preserving helper extraction in `hydrology_phase_erod19.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API, module visibility, dependency, or caller behavior changes.
- No runtime symbol, alias, unit, formula, float expression order, parser
  compatibility, typed status/error, dispatch order, or science-contract
  behavior changes.
- No unrelated hydrology, erosion, or watershed cleanup.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts under `artifacts/`.

## Intended Write Set

- `docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`
- Focused tests under existing hydrology/contract test paths if
  characterization is required before production refactor.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`
- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the current target function from live metrics.
- Record protected symbols, formulas, statuses, units, and phase behavior.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits.
- Add targeted characterization before production decomposition if current tests
  do not freeze selected branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers as needed.
- Preserve formula operand order, public signatures, typed status behavior,
  dispatch order, and science-contract behavior.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR23 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor when
  characterization is added.
- No public API, formula, runtime symbol, alias, unit, stable status/error,
  parser compatibility, or science-contract behavior change is introduced.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification artifacts. Subagent spawning is not required for closure; if
separate subagents are unavailable or not authorized in the current tool policy,
the executing agent must perform equivalent independent local reviews and record
that path.

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: private Rust helper extraction and focused characterization only;
  no new subprocess, network, serialization format, unsafe, dependency, or
  public API.

## Closure Summary

Ran: CQR23 reduced
`Wb11HydrologyKernel::run_erod19_route_segment_migration` from CRAP
`351.9234211799049`, CC `79`, and coverage `64.76868327402136%` to CRAP
`9.00460855712335`, CC `9`, and coverage `96.15384615384616%`.

Ran: every newly extracted helper is CRAP `14.787398726851855` or lower. The
pre-existing out-of-scope `Wb11HydrologyKernel::erod19_depend` row remains CRAP
`87.98408081839372`; this package did not modify that helper.

Ran: target-file line coverage improved from `476/647` (`73.57%`) to `766/904`
(`84.73%`), and function coverage improved from `15/17` (`88.24%`) to `34/36`
(`94.44%`). Coverage is still below the ADR-0021 `90%` line threshold, recorded
as a warning rather than a blocker because CQR23 is a scoped complexity package,
not a module coverage package.
