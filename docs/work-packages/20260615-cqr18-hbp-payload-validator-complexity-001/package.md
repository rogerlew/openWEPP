# CQR18 - HBP Payload Validator Complexity Refactor

Status: complete

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR18 target in
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`,
originally tracked as rank 12 with snapshot CRAP `456`, CC `80`, and coverage
`61%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The HBP payload validator protects parser compatibility, payload CRC and bounds
checks, event payload decoding, state snapshot registry conformance, required
state obligations, and typed HBP error IDs. CQR18 must reduce local complexity
without changing accepted payloads, rejected payloads, error IDs, or binary
format compatibility.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target-function identity, line counts,
  suppression census, public API parity, behavior equivalence, and final gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused characterization before production refactor when needed.
- Private behavior-preserving helper extraction in `payload_validator.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API, module visibility, dependency, or caller behavior changes.
- No HBP schema layout, payload CRC, payload bounds, event-kind behavior,
  state snapshot schema, required-state registry, parser compatibility, or HBP
  error ID changes.
- No unrelated HBP parser, bridge, runner, or watershed cleanup.

## Deliverables

1. Source refactor:
   - `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts under `artifacts/`.

## Intended Write Set

- `docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
- Focused tests under existing HBP parser test paths if characterization is
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
- `docs/codex_exec_plans.md`
- `crates/AGENTS.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the current target function from live metrics.
- Record protected schema, CRC, bounds, event, state registry, and error-ID
  behavior.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits.
- Add targeted characterization before production decomposition if current tests
  do not freeze selected branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers as needed.
- Preserve byte decoding order, cursor advancement, payload compatibility,
  typed error IDs, and exact validation messages.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR18 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor when
  characterization is added.
- No public API, binary HBP schema, CRC behavior, parser compatibility, required
  state obligations, or typed HBP error IDs change.
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
