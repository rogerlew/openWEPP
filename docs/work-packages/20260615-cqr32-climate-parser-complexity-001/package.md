# CQR32 - Climate Parser Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor

## Objective

Close the current CQR32 target in
`crates/openwepp-input-contract/src/parsers/climate.rs`, originally tracked as
rank 26 with snapshot CRAP `240`, CC `15`, and coverage `0%`, so the live
target function and any newly extracted helpers have CRAP `<= 30`.

## Rationale

The climate parser preserves CLIGEN climate-file compatibility and emits typed
parser structures consumed by runtime projection. This package is limited to
characterization and private behavior-preserving decomposition of the scoped
high-CRAP parser target.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, before/after CRAP, target identity,
  line counts, suppression census, parser equivalence, reviews,
  verification, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Characterization coverage before production refactor when needed.
- Behavior-preserving private helper extraction for the scoped target if live
  metrics require it.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public parser API changes.
- No grammar, token order, compatibility mode, typed error, error variant,
  field name, alias, unit, parser-output shape, runtime-projection, or
  kernel-facing semantic changes.
- No fallback wrappers or default-and-proceed behavior for required parser
  inputs.
- No dependency changes.
- No unrelated parser cleanup, broad parser rewrite, file split, or science
  contract change.

## Kernel / Parser Acceptance Gate

This package touches parser authority that feeds kernel-facing runtime
projection. The current-scope acceptance gate is parser behavior and output
surface equivalence:

- preserve parser public functions and result types;
- preserve accepted legacy fixture behavior and strict fail-closed paths;
- preserve parser token order, branch selection, count/cardinality guards,
  field names, error variants, and emitted structure shape;
- preserve downstream runtime/kernel-facing meanings of parsed climate fields;
- stop and hold if a production edit requires changing science authority,
  thresholds, units, aliases, symbols, typed guards, parser compatibility, or
  public API behavior.

## Deliverables

1. Source refactor if live metrics require it:
   - `crates/openwepp-input-contract/src/parsers/climate.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr32-quality-plan-report.md`
   - `artifacts/cqr32-crap-before.md`
   - `artifacts/cqr32-crap-after.md`
   - `artifacts/cqr32-coverage-closure.md`
   - `artifacts/cqr32-parser-equivalence.md`
   - `artifacts/cqr32-implementation-and-test-evidence.md`
   - `artifacts/cqr32-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr32_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/lcov_before.info`
   - `artifacts/lcov_after.info`
   - `artifacts/crap_before.json`
   - `artifacts/crap_after.json`

## Intended Write Set

- `docs/work-packages/20260615-cqr32-climate-parser-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-input-contract/src/parsers/climate.rs` only if live metrics
  require behavior-preserving refactor.
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
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the live target function from current metrics.
- Record protected parser APIs, compatibility controls, typed errors,
  cardinality/date guards, field names, and output structure.

### Phase B - Precondition and Focused Characterization

- Run existing focused parser tests before production edits when production
  edits are needed.
- Add characterization before refactor only if current tests do not freeze the
  selected branches.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers only if live metrics require edits.
- Preserve public parser APIs, parse order, branch behavior, typed errors,
  compatibility controls, count/cardinality validation, date validation, and
  output structure.

### Phase D - Validation and Evidence

- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr32-climate-parser-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR32 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Public parser APIs, grammar, token order, compatibility controls, typed
  error behavior, error variants, field names, units, parser-output shape, and
  runtime/kernel-facing meanings are unchanged.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification artifacts. Subagent spawning is not required for closure; if
separate subagents are unavailable or not authorized in the current tool
policy, the executing agent must perform equivalent independent local reviews
and record that path.

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
