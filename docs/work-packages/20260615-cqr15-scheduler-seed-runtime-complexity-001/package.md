# CQR15 - Scheduler Seed Runtime Complexity Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR15 target in
`crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`,
originally tracked as rank 9 with snapshot CRAP `581`, CC `94`, and coverage
`62%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The scheduler seed/runtime module publishes runtime seed symbols consumed by
hydrology and trace execution. The likely live target is the large WB11 runtime
surface seeding routine, which currently carries a `too_many_lines`
suppression. CQR15 must reduce complexity without changing runtime symbol
names, lane policy, typed guard behavior, unit meanings, state seeding formulas,
or science-contract behavior.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target-function identity, line counts,
  suppression census, public API parity, behavior equivalence, and
  kernel-profile compliance.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused characterization before production refactor when needed.
- Private helper extraction inside
  `scheduler_seed_and_runtime.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API, module visibility, or caller behavior changes.
- No runtime symbol name, alias, unit, lane selection, state seeding formula,
  guard detail string, stable error variant, parser compatibility, output
  formula, float expression order, or science-contract behavior change.
- No unrelated scheduler trace cleanup, file split, or broad runner rewrite.
- No dependency changes.

## Acceptance Gate

This package is kernel-affecting because it edits runtime projection/seeding that
controls hydrology kernel inputs and branches. The acceptance gate is
behavior-preserving runtime-surface parity:

- preserve runtime seed symbols and lane-selection semantics;
- preserve typed `HillslopeCliError` failure behavior and guard detail strings;
- preserve WB11/WB12/WB13/WB14/WB17/WB18/WB19/WB20 runtime publication and seed
  formulas;
- preserve float expression order for existing formulas unless only moved into
  a private helper with identical operands and order;
- stop and hold if a production edit requires changing science-contract
  authority, runtime symbol meaning, or kernel-facing behavior.

## Deliverables

1. Source refactor:
   - `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr15-quality-plan-report.md`
   - `artifacts/cqr15-public-api-surface-parity-report.md`
   - `artifacts/cqr15-crap-before.md`
   - `artifacts/cqr15-crap-after.md`
   - `artifacts/cqr15-coverage-closure.md`
   - `artifacts/cqr15-behavior-equivalence.md`
   - `artifacts/cqr15-implementation-and-test-evidence.md`
   - `artifacts/cqr15-line-count-governance-checklist.md`
   - `artifacts/cqr15-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr15_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/lcov_before.info`
   - `artifacts/lcov_after.info`
   - `artifacts/crap_before.json`
   - `artifacts/crap_after.json`

## Intended Write Set

- `docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
- Focused tests under existing runner test paths if characterization is
  required before production refactor.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/codex_exec_plans.md`
- `crates/AGENTS.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the current target function from live metrics.
- Record runtime symbol, lane policy, error, formula, and science-contract
  protected boundaries.

### Phase B - Precondition and Focused Characterization

- Run existing focused scheduler/runner tests before production edits.
- Add targeted characterization before production decomposition if current tests
  do not freeze selected branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers as needed.
- Preserve statement order where observable, short-circuiting, typed errors,
  runtime symbols, lane policy, formula operand order, and public API.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR15 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor when
  characterization is added.
- No public API, runtime symbol, lane policy, stable error variant, formula,
  float expression order, parser compatibility, or science-contract behavior
  change is introduced.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- Touched Rust line-count WARN is dispositioned because the target file starts
  above `2000` lines.
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
