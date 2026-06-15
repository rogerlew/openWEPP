# CQR09 - Decomposition Equations CRAP Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current highest-CRAP target function in
`crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`,
originally tracked as CQR rank 3 with snapshot CRAP `1497`, CC `79`, and
coverage `39%`, so the target function and any newly extracted helpers have
CRAP `<= 30`.

## Rationale

`07_decomposition_equations.rs` owns the plant/decomposition transition
equation helpers used by the hillslope scheduler. The file is kernel-affecting
because it computes and guards decomposition transition payloads. This package
is limited to test-first characterization and private helper extraction that
reduces complexity while preserving process behavior.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against the package LCOV.
- Supporting metrics: before/after LCOV, target-file coverage, target function
  identity, target line count, suppression census, and `.rs` line-count
  governance.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused characterization for decomposition equation behavior if current tests
  do not already freeze the selected target branches.
- Private helper extraction inside `07_decomposition_equations.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, reviews, verification, gate evidence, disposition, and
  worker handoff.

## Excluded Scope / Protected Boundaries

- No public API changes.
- No science-contract, formula, constant, threshold, symbol, alias, unit,
  parser-compatibility, typed guard, error ID, or scheduler behavior change.
- No float expression regrouping, accumulation-order change, or short-circuit
  behavior change.
- No dependency changes.
- No unrelated lint cleanup, dead-code removal, file split, or broad module
  rewrite.

## Kernel Acceptance Gate

This package is kernel-affecting but behavior-preserving. The current-scope
acceptance gate is numeric and guard-surface equivalence:

- preserve every decomposition equation formula and float expression grouping;
- preserve all `HillslopeDecompositionBoundaryError` variants, stable error
  IDs, symbols, reason strings, and fail-closed paths;
- preserve public scheduler transition payload behavior;
- stop and hold if a production edit requires changing science authority,
  thresholds, units, aliases, symbols, typed guards, or public API behavior.

Independent output reconstruction is not in scope because no conservation
publication formula is intentionally changed.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr09-quality-plan-report.md`
   - `artifacts/cqr09-public-api-surface-parity-report.md`
   - `artifacts/cqr09-crap-before.md`
   - `artifacts/cqr09-crap-after.md`
   - `artifacts/cqr09-coverage-closure.md`
   - `artifacts/cqr09-numeric-equivalence.md`
   - `artifacts/cqr09-implementation-and-test-evidence.md`
   - `artifacts/cqr09-line-count-governance-checklist.md`
   - `artifacts/cqr09-kernel-profile-compliance.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr09_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/lcov_before.info`
   - `artifacts/lcov_after.info`
   - `artifacts/crap_before.json`
   - `artifacts/crap_after.json`

## Intended Write Set

- `docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
- Focused tests under existing
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/**` or
  `tests/integration/parser_runtime_seam_integration/**` if characterization is
  required before production refactor.

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
- Record public API, numeric, guard, symbol, alias, and scheduler-payload
  protected boundaries.

### Phase B - Precondition and Focused Characterization

- Run existing focused decomposition/scheduler tests before production edits.
- Add targeted characterization before production decomposition if the selected
  function's covered branches are not already exact enough.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive whole branches or validation clusters into private helpers.
- Preserve exact statement order, float expression order/grouping,
  short-circuiting, typed errors, symbols, reason strings, and public behavior.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts, including Gate Evidence
  Non-Deferral, numeric-equivalence, CRAP target, coverage non-regression, and
  line-count-governance checks.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR09 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor.
- No public API, formula, threshold, typed guard, error ID, symbol, alias, unit,
  parser-compatibility, output formula, or scheduler behavior change is
  introduced.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- No touched non-exempt `.rs` file is at or above 3000 lines.
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
