# CQR28 - Plant Percolation Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor

## Objective

Close the current CQR28 target in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`,
originally tracked as rank 22 with snapshot CRAP `282`, CC `91`, and
coverage `72%`, so the live target function and any newly extracted helpers
have CRAP `<= 30`.

## Rationale

The target file owns WB17 plant root uptake and adjacent WB18 aggregate
storage/percolation helpers. It is kernel-affecting code. CQR28 must reduce or
prove closure of complexity without changing science-contract behavior, public
API, runtime symbols, units, formulas, float expression order, typed guards,
or writeback behavior.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, before/after CRAP, target identity,
  line counts, suppression census, kernel-profile compliance, behavior
  equivalence, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Characterization coverage before production refactor where current tests do
  not pin selected branches.
- Behavior-preserving private helper extraction for the scoped target if live
  metrics require it.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API changes.
- No science-contract threshold, unit, alias, symbol, formula, float expression
  order, typed guard, writeback, parser compatibility, or output behavior
  changes.
- No dependency changes.
- No unrelated hydrology, plant, percolation, lateral, drainage, output, or
  scheduler cleanup.

## Kernel Acceptance Gate

This package is kernel-affecting because WB17 plant root uptake mutates layer
storage, aggregate `wb11_soil_water`, `ET`/`Ep`, `Ws`, and layer
`UPi`/`Ui` writebacks consumed by plant and water-balance contracts. The
acceptance gate is strict behavior preservation:

- preserve all public and crate-visible signatures;
- preserve runtime symbols, aliases, units, and writeback ordering;
- preserve stable guard IDs, typed errors, and domain checks;
- preserve all formulas and float expression order;
- stop and hold if closure requires changing science authority.

## Intended Write Set

- `docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`
  only if live metrics require a behavior-preserving refactor.
- Focused tests only if characterization coverage is required before refactor.

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
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the live target function from current metrics.
- Record protected runtime symbol, guard, output, formula, and writeback
  surfaces.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits when production edits are
  needed.
- Add characterization before refactor only if current tests do not freeze the
  selected branches.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers only if live metrics require edits.
- Preserve formula operand order, writeback order, typed errors, symbols,
  aliases, units, and science-contract behavior.

### Phase D - Validation and Evidence

- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr28-plant-percolation-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR28 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- No public API, runtime symbol, alias, unit, formula, threshold, typed guard,
  writeback, parser compatibility, output, or science-contract behavior change
  is introduced.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification before disposition.

## Final Outcome

Ran: CQR28 closed the scoped live target
`Wb11HydrologyKernel::run_percolation` from CRAP
`281.82979375564685` to `17.19373252009578`. All newly extracted helpers are
CRAP `<= 22.896222121074196`.

Ran: The refactor is private behavior-preserving decomposition only. It keeps
public API, crate-visible kernel entry points, runtime symbols, units, formulas,
float expression order inside formulas, typed guard/error IDs, writeback order,
parser compatibility, and science-contract behavior unchanged.

Warnings:

- `cargo crap` reports 126 LCOV source-map warnings on both before/after runs;
  the target file is represented in LCOV and target/helper CRAP closure is
  computed from the package LCOV files.
- Two pre-existing same-file rows remain above CRAP `30` outside this package
  target: `resolve_effective_wb18_frozen_depth` and `run_plant_root_uptake`.
