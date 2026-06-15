# CQR12 - Depletion Irrigation Runtime CRAP Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR12 target in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`,
originally tracked as rank 6 with snapshot CRAP `1122`, CC `33`, and coverage
`0%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The target runtime-input module seeds parser output for snow, frost, and
irrigation into kernel-facing runtime surfaces. CQR10 closed the fixed-date
irrigation target and deliberately left the depletion-scheduled irrigation
seeder as the next ranked row. CQR12 is limited to characterization and private
behavior-preserving decomposition of that depletion projection path.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target-function identity, line counts,
  suppression census, public API parity, numeric/symbol equivalence, and
  line-count governance.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused depletion irrigation runtime characterization before production
  refactor when needed.
- Private helper extraction inside `04_snow_frost_irrigation.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API changes.
- No science-contract, formula, constant, threshold, symbol, alias, unit,
  parser-compatibility, typed guard, error ID, scheduler behavior, or kernel
  branch-selection change.
- No float expression regrouping, accumulation-order change, or short-circuit
  behavior change.
- No dependency changes.
- No unrelated lint cleanup, dead-code removal, file split, or broad module
  rewrite.

## Kernel Acceptance Gate

This package is kernel-affecting because depletion-scheduled irrigation runtime
projection controls kernel-facing irrigation state. The current-scope
acceptance gate is projection-surface and guard equivalence:

- preserve every `irrigation.depletion.*` symbol name and value meaning;
- preserve all `HillslopeRuntimeInputError` variants, field names, allowed
  strings, and fail-closed paths;
- preserve parser compatibility and depletion period iteration/order;
- preserve sprinkler/furrow value checks and insertion order where observable;
- stop and hold if a production edit requires changing science authority,
  thresholds, units, aliases, symbols, typed guards, public API behavior, or
  runtime/kernel projection meaning.

Independent output reconstruction is not in scope because no conservation
publication formula is intentionally changed.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr12-quality-plan-report.md`
   - `artifacts/cqr12-public-api-surface-parity-report.md`
   - `artifacts/cqr12-crap-before.md`
   - `artifacts/cqr12-crap-after.md`
   - `artifacts/cqr12-coverage-closure.md`
   - `artifacts/cqr12-numeric-equivalence.md`
   - `artifacts/cqr12-kernel-profile-compliance.md`
   - `artifacts/cqr12-implementation-and-test-evidence.md`
   - `artifacts/cqr12-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr12_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/lcov_before.info`
   - `artifacts/lcov_after.info`
   - `artifacts/crap_before.json`
   - `artifacts/crap_after.json`

## Intended Write Set

- `docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
- Focused tests under existing crate or integration test paths if
  characterization is required before production refactor.

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
- Identify the current target function from live metrics.
- Record public API, numeric, guard, symbol, alias, parser, and
  kernel-projection protected boundaries.

### Phase B - Precondition and Focused Characterization

- Run existing focused depletion irrigation tests before production edits.
- Add targeted characterization before production decomposition if current tests
  do not freeze the selected branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive validation, header seeding, period seeding, sprinkler, and
  furrow clusters into private helpers as needed.
- Preserve exact statement order, float expression order/grouping,
  short-circuiting, typed errors, symbols, field names, and public behavior.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts, including Gate Evidence
  Non-Deferral, numeric-equivalence, CRAP target, coverage non-regression, and
  line-count-governance checks.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR12 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor.
- No public API, formula, threshold, typed guard, error ID, symbol, alias, unit,
  parser-compatibility, output formula, scheduler behavior, or kernel
  projection behavior change is introduced.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above `2000` lines unless a WARN disposition
  is recorded with owner and follow-on intent.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification artifacts. Subagent spawning is not required for closure; if
separate subagents are unavailable or not authorized in the current tool policy,
the executing agent must perform equivalent independent local reviews and record
that path.

## Instruction Precedence

Package-required gates override ambient test/validation-skip guidance,
including: `UNLESS you are explicitly requested to do so, NEVER run tests or
validate your work.`

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: private Rust helper extraction and focused characterization only;
  no new subprocess, network, serialization format, unsafe, dependency, or
  public API.
