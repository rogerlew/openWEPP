# CQR26 - Lateral Drainage Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor / live-metric closure

## Objective

Close the current CQR26 target in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`,
originally tracked as rank 20 with snapshot CRAP `300`, CC `122`, and
coverage `77%`, so the live target function and any newly extracted helpers
have CRAP `<= 30`.

## Rationale

The lateral-drainage phase owns WB19 lateral transfer, drainage, and related
hydrology handoff helpers. This is kernel-affecting code. The CQR26 package
must reduce or prove closure of complexity without changing science-contract
behavior, public API, runtime symbols, units, formulas, typed guards, or
writeback behavior.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, before/after CRAP, target identity,
  line counts, suppression census, kernel-profile compliance, behavior
  equivalence, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Live-metric proof that the snapshot row is already closed, if confirmed.
- Behavior-preserving private helper extraction only if live metrics require it.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API changes.
- No science-contract threshold, unit, alias, symbol, formula, float expression
  order, typed guard, writeback, or publication behavior changes.
- No dependency changes.
- No unrelated hydrology, drainage, lateral-transfer, parser, output, or
  scheduler cleanup.

## Kernel Acceptance Gate

This package is kernel-affecting because WB19 lateral drainage and related
hydrology helpers participate in state mutation, runoff, drainage, and
publication surfaces. The acceptance gate is strict behavior preservation:

- preserve all public and crate-visible signatures;
- preserve runtime symbols, aliases, units, and writeback ordering;
- preserve stable guard IDs, typed errors, and domain checks;
- preserve all formulas and float expression order unless no production edit is
  performed;
- stop and hold if closure requires changing science authority.

## Intended Write Set

- `docs/work-packages/20260615-cqr26-lateral-drainage-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
  only if live metrics require a behavior-preserving refactor.

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the live target function from current metrics.
- Record protected runtime symbol, guard, output, and writeback surfaces.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits when production edits are
  needed.
- Add characterization before refactor only if current tests do not freeze the
  selected branches.

### Phase C - Behavior-Preserving Decomposition

- If live metrics require edits, extract cohesive private helpers only.
- Preserve formula operand order, writeback order, typed errors, symbols,
  aliases, units, and science-contract behavior.

### Phase D - Validation and Evidence

- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr26-lateral-drainage-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR26 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- No public API, runtime symbol, alias, unit, formula, threshold, typed guard,
  writeback, parser compatibility, or science-contract behavior change is
  introduced.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification before disposition.

## Closeout Summary

Ran: live metrics proved the current target file was already closed before any
production edit. The highest target-file row in both before and after CRAP
reports is
`Wb11HydrologyKernel::wb19_lateral_transfer_inputs` at line `172`, CC `18.0`,
coverage `70.23809523809523%`, and CRAP `26.541362973760947`.

Ran: no target-file CRAP rows exceed `30` in either report.

Static: no production Rust file was changed for this package. No public API,
runtime symbol, alias, unit, formula, float expression order, typed guard,
writeback, parser, output, or science-contract behavior was modified.

Warnings: `cargo crap` emitted LCOV source-map warnings for 126 workspace
test/support source files, while the target file was present in LCOV. The
target file is `2527` lines, below the hard `3000` line ceiling but above the
older caution threshold; it was not edited.
