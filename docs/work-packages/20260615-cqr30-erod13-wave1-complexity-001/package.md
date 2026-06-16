# CQR30 - EROD13 Wave-1 Complexity Closure

Status: complete

Package type: code-quality refactor

## Objective

Close the current CQR30 target in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`,
originally tracked as rank 24 with snapshot CRAP `265`, CC `81`, and coverage
`70%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The target owns EROD13 Wave-1 erosion-core runtime behavior. This is
kernel-affecting code governed by `SC-SED-001` plus the active EROD13 producer,
consumer, and boundary-carry addenda. Changes must preserve public API, guard
codes, symbols, output formulas, branch behavior, float expression order, and
science-contract behavior.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Characterization coverage before production refactor when needed.
- Behavior-preserving private decomposition of the scoped metric target only.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No formula, branch-threshold, float expression-order, or output-symbol
  changes.
- No public API, typed guard, error ID, parser compatibility, alias, unit, or
  science-contract behavior changes.
- No edits to EROD14, EROD18/19, runner activation, runtime projection, or
  watershed routing unless required only for characterization.

## Intended Write Set

- `docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`
  only if live metrics require behavior-preserving refactor.
- Focused tests only if characterization coverage is required before refactor.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Exit Criteria

- Current CQR30 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Public API, typed guards, error IDs, symbols, units, parser compatibility,
  output formulas, float expression order, and science-contract behavior are
  preserved.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.
