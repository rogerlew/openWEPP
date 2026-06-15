# CQR19 - Watershed Runtime Types Complexity Refactor

Status: complete pending package commit and push

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR19 target in
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`,
originally tracked as rank 13 with snapshot CRAP `420`, CC `20`, and coverage
`0%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The watershed runtime input type module owns typed runtime seam error IDs and
display messages for channel, impoundment, and climate runtime adaptation.
CQR19 must reduce local complexity without changing error IDs, public enum
variants, display text, runtime seam semantics, or parser/runtime compatibility.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target identity, line counts,
  suppression census, public API parity, behavior equivalence, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused characterization before production refactor when needed.
- Private behavior-preserving helper extraction in `types.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public enum, variant, field, code string, display text, dependency, or
  caller behavior changes.
- No channel, impoundment, climate projection, parser compatibility, runtime
  symbol, unit, or science-contract behavior changes.
- No unrelated watershed runtime cleanup.

## Deliverables

1. Source refactor:
   - `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts under `artifacts/`.

## Intended Write Set

- `docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`
- Focused tests under existing watershed runtime test paths if
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
- Record protected enum, error-code, display, runtime-seam, and compatibility
  behavior.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits.
- Add targeted characterization before production decomposition if current tests
  do not freeze selected display/error branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers as needed.
- Preserve public type surface, variant matching semantics, error IDs, display
  messages, and runtime seam behavior.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR19 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor when
  characterization is added.
- No public API, enum variant, error ID, display text, runtime symbol, parser
  compatibility, unit, or science-contract behavior changes.
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
