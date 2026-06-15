# CQR04 - Watershed Routing Kernel Complexity Refactor

Status: completed-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
so high-risk watershed channel routing helpers no longer concentrate WS18
hydraulic geometry, WS18 transport capacity, WS20 case-1/2 segment routing,
WS21 case-3/4 continuation, WS22/WS23/WS24 detach-transition paths, and
state-symbol projection in long high-CRAP functions.

## Rationale

ADR-0021 names `ws20_route_case12_segment_family` as a complexity-risk backlog
item. The file is kernel math under WS10/WS11/WS20-WS24 watershed routing
authority, and it already carries several `#[allow(clippy::too_many_lines)]`
attributes. This package reduces CRAP/cyclomatic complexity by extracting
cohesive private helpers while preserving current routing behavior, typed guard
families, symbol names, constants, thresholds, and expression grouping.

## Quality Dimension

- Dimension: cyclomatic complexity / CRAP reduction for
  `lib_mod/kernel/routing.rs`.
- Closure metric: every eligible function in `routing.rs` has CRAP `<= 30`
  after the refactor.
- Supporting lint-ratchet outcome: remove obsolete
  `#[allow(clippy::too_many_lines)]` suppressions from this file when the
  decomposed functions satisfy clippy.
- Supporting metrics: record before/after function length and target coverage.

## Included Scope

- Private helper extraction inside `routing.rs`.
- Private context structs if needed to pass segment routing state explicitly.
- Focused characterization only if measured pre-refactor coverage is below the
  science-tier safety-net threshold for this kernel module.
- Package catalog update in `docs/work-packages/README.md`.
- Focused watershed routing tests and required Rust closure gates.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No science-contract amendments.
- No routing formula, constant, threshold, tolerance, branch predicate,
  canonicalization, or guard strictness changes.
- No public crate API changes.
- No edits to parser projection, runner orchestration, output writers, or
  impoundment logic outside this routing module.
- No module/file split beyond this file; this package targets intra-module
  function decomposition only.
- No unrelated naming cleanup, dead-code deletion, or broad lint ratchet outside
  the target file.

## Deliverables

1. Source refactor:
   - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
2. Optional focused tests if required by coverage precondition:
   - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
   - or crate-local watershed orchestrator tests if a narrower existing test
     module is the better fit
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr04-quality-plan-report.md`
   - `artifacts/cqr04-public-api-surface-parity-report.md`
   - `artifacts/cqr04-function-length-before.md`
   - `artifacts/cqr04-function-length-after.md`
   - `artifacts/cqr04-crap-before.md`
   - `artifacts/cqr04-crap-after.md`
   - `artifacts/cqr04-coverage-closure.md`
   - `artifacts/cqr04-routing-equivalence.md`
   - `artifacts/cqr04-implementation-and-test-evidence.md`
   - `artifacts/cqr04-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr04_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
- Optional focused tests only if baseline coverage requires characterization:
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `crates/AGENTS.md`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `routing.rs`.
- Capture public/internal caller surface expectations for target helpers.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record package scope and write-set boundaries.

### Phase B - Precondition and Focused Characterization

- Run focused WS10/WS11 channel routing tests.
- If focused characterization fails before edits, stop and record a blocker.
- If coverage is below science-tier closure thresholds, add focused
  characterization before production decomposition.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive routing stages and branch bodies into private helpers.
- Preserve guard order where externally observable through returned typed guard
  errors.
- Preserve exact arithmetic expression grouping and accumulation order.
- Remove target-file `too_many_lines` suppressions only when clippy accepts the
  decomposed functions.

### Phase D - Validation and Evidence

- Run focused checks after the refactor.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Re-run coverage/LCOV and `cargo-crap`; target-module CRAP rows must be
  `<= 30`.
- Record command outcome and explicit exit code for every gate.

### Phase E - Review, Verification, and Disposition

- Complete dual review artifacts with finding disposition.
- Complete dual verification artifacts, including gate legitimacy, routing
  equivalence, metric target, and line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- Every eligible function in `routing.rs` has CRAP `<= 30`.
- High-risk routing functions are decomposed into cohesive private helpers
  without changing WS routing behavior, typed guard IDs, symbol names, branch
  predicates, arithmetic expression grouping, or public APIs.
- Focused WS10/WS11 routing characterization passes before and after refactor.
- Target coverage does not regress below science-tier closure thresholds
  (`>= 90%` line and region) unless a reviewed, scoped hold is recorded.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.

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
- rationale: private Rust kernel helper extraction only; no new subprocess,
  network, serialization format, unsafe, dependency, or public API.
