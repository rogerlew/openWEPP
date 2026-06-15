# CQR02 - HBP Layout Parser Complexity Refactor

Status: complete

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs` so
`parse_layout` no longer concentrates the HBP header, dimension, registry,
directory, schema-1 footer, schema-2 block-table, checksum, and raw-block
validation branches in one high-CRAP function.

## Rationale

`parse_layout` is an 841-line parser function with many independent validation
clusters. The parser behavior is already covered by the HBP parser contract
suite, so the code-quality target is a behavior-preserving intra-function
decomposition: extract cohesive parser stages into private helpers without
changing binary layout order, error codes, error messages, checksum windows,
schema branch behavior, public parse APIs, or typed fail-closed posture.

## Quality Dimension

- Dimension: cyclomatic complexity / CRAP reduction for
  `layout_parser.rs`.
- Closure metric: every eligible function in
  `layout_parser.rs` has CRAP `<= 30` after the refactor.
- Supporting metrics: record before/after function-length and target coverage.

## Included Scope

- Private helper extraction inside `layout_parser.rs`.
- Private helper structs if needed to pass parser-stage context explicitly.
- Pre-refactor characterization tests in
  `tests/integration/infile_hbp_parser_contract.rs` if baseline coverage is
  below the glue-tier closure threshold.
- Package catalog update in `docs/work-packages/README.md`.
- Focused HBP parser characterization and required Rust closure gates.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No parser behavior, binary layout, checksum window, schema support, error
  code, or error message changes.
- No public API shape change for HBP parser entrypoints in
  `crates/openwepp-input-contract/src/parsers/hbp/mod.rs`.
- No new compatibility fallback, silent default, broad error swallowing, or
  guard loosening.
- No edits to HBP payload validation, path resolution, or public type surfaces.
- Test edits are limited to HBP layout-parser characterization required by the
  measured coverage precondition before decomposition.
- No removal of the HBP module-level `clippy::too_many_lines` allowance; other
  HBP functions remain outside this package's single CRAP dimension.

## Deliverables

1. Source refactor:
   - `crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr02-quality-plan-report.md`
   - `artifacts/cqr02-public-api-surface-parity-report.md`
   - `artifacts/cqr02-function-length-before.md`
   - `artifacts/cqr02-function-length-after.md`
   - `artifacts/cqr02-crap-before.md`
   - `artifacts/cqr02-crap-after.md`
   - `artifacts/cqr02-coverage-closure.md`
   - `artifacts/cqr02-parser-equivalence.md`
   - `artifacts/cqr02-implementation-and-test-evidence.md`
   - `artifacts/cqr02-parser-profile-compliance-checklist.md`
   - `artifacts/cqr02-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr02_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr02-hbp-layout-parser-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`
- `tests/integration/infile_hbp_parser_contract.rs`

## Dependencies

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `crates/AGENTS.md`
- `tests/integration/infile_hbp_parser_contract.rs`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `layout_parser.rs`.
- Capture public HBP parser API surface expectations from `hbp/mod.rs`.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record package scope and write-set boundaries.

### Phase B - Precondition and Focused Characterization

- Run the existing focused HBP parser contract suite:
  `cargo test --test infile_hbp_parser_contract`.
- If focused HBP characterization fails before edits, stop and record a blocker
  instead of refactoring under a red precondition.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive parser stages from `parse_layout` into private helpers in
  `layout_parser.rs`.
- Preserve byte-read order, cursor advancement, branch order, checksum windows,
  error codes, and error messages.
- Keep public parser entrypoints and call sites stable.

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
- Complete dual verification artifacts, including gate legitimacy,
  parser-equivalence, metric target, and line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- Every eligible function in `layout_parser.rs` has CRAP `<= 30`.
- `parse_layout` is decomposed into cohesive private helpers without changing
  parser behavior, typed error codes, or public HBP parse APIs.
- Focused HBP characterization passes before and after the refactor.
- Target coverage does not regress below glue-tier closure thresholds
  (`>= 85%` line and region) unless a reviewed, scoped hold is recorded.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.

## Subagent Requirement

Subagent authorization: this package explicitly authorizes
spawning/delegating to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is limited to the package
artifact files. If subagents are unavailable or tool policy does not allow
delegation from this turn, the executing agent must perform equivalent
independent local reviews and record that path.

## Instruction Precedence

Package-required gates override ambient test/validation-skip guidance,
including: `UNLESS you are explicitly requested to do so, NEVER run tests or
validate your work.`

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: private Rust parser helper extraction only; no new subprocess,
  network, serialization format, unsafe, dependency, or public parser API.
