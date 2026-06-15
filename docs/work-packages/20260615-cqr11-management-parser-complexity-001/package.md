# CQR11 - Management Parser CRAP Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR11 target in
`crates/openwepp-input-contract/src/parsers/management.rs`, originally tracked
as rank 5 with snapshot CRAP `1406`, CC `37`, and coverage `0%`, so the live
target function and any newly extracted helpers have CRAP `<= 30`.

## Rationale

The management parser preserves legacy management-file compatibility and emits
typed parser structures consumed by runtime projection. This package is limited
to characterization and private behavior-preserving decomposition of the scoped
high-CRAP parser target.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target-function identity, line counts,
  suppression census, parser compatibility evidence, public API parity, and
  line-count governance.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused parser characterization before production refactor when needed.
- Private helper extraction inside `management.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public parser API changes.
- No grammar, token order, compatibility, typed error, field, alias, unit,
  parser-output shape, runtime-projection, or kernel-facing semantic changes.
- No fallback wrappers or default-and-proceed behavior for required parser
  inputs.
- No dependency changes.
- No unrelated lint cleanup, broad parser rewrite, file split, or science
  contract change.

## Kernel / Parser Acceptance Gate

This package touches parser authority that feeds kernel-facing runtime
projection. The current-scope acceptance gate is parser behavior and output
surface equivalence:

- preserve parser public functions and result types;
- preserve accepted legacy fixture behavior and strict fail-closed paths;
- preserve parser token order, branch selection, count/cardinality guards,
  field names, error IDs, and emitted structure shape;
- preserve all downstream runtime/kernel-facing meanings of parsed management
  fields;
- stop and hold if a production edit requires changing science authority,
  thresholds, units, aliases, symbols, typed guards, parser compatibility, or
  public API behavior.

## Deliverables

1. Source refactor:
   - `crates/openwepp-input-contract/src/parsers/management.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr11-quality-plan-report.md`
   - `artifacts/cqr11-public-api-surface-parity-report.md`
   - `artifacts/cqr11-crap-before.md`
   - `artifacts/cqr11-crap-after.md`
   - `artifacts/cqr11-coverage-closure.md`
   - `artifacts/cqr11-parser-equivalence.md`
   - `artifacts/cqr11-kernel-profile-compliance.md`
   - `artifacts/cqr11-implementation-and-test-evidence.md`
   - `artifacts/cqr11-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr11_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/lcov_before.info`
   - `artifacts/lcov_after.info`
   - `artifacts/crap_before.json`
   - `artifacts/crap_after.json`

## Intended Write Set

- `docs/work-packages/20260615-cqr11-management-parser-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-input-contract/src/parsers/management.rs`
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
- Record public parser API, compatibility, typed-error, field, cardinality, and
  runtime/kernel protected boundaries.

### Phase B - Precondition and Focused Characterization

- Run existing focused parser tests before production edits.
- Add targeted characterization before production decomposition if current
  tests do not freeze the selected branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive parser subroutines into private helpers as needed.
- Preserve exact parse order, branch behavior, typed errors, count/cardinality
  validation, output structure, and public behavior.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr11-management-parser-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts, including Gate Evidence
  Non-Deferral, parser-equivalence, CRAP target, coverage non-regression, and
  line-count-governance checks.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR11 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor.
- No public parser API, typed error, error ID, field, alias, unit, grammar,
  parser-output shape, parser compatibility, runtime projection meaning, or
  kernel-facing behavior change is introduced.
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
- rationale: private Rust parser helper extraction and focused
  characterization only; no new subprocess, network, serialization format,
  unsafe, dependency, or public API.
