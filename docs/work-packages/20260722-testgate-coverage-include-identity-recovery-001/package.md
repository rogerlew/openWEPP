# TESTGATE Coverage Include Identity Recovery

Package: `20260722-testgate-coverage-include-identity-recovery-001`
Status: `ACTIVE`
Defect: `RTR-043`
Cause: `GATE-COVERAGE-INCLUDED-SOURCE-IDENTITY-LOSS`

## Objective

Close RTR-043 by replacing the inline `include!` wiring for verifier coverage
tests with source-identity-preserving module wiring, then prove the included
file receives attributable LLVM coverage without changing verifier or test
behavior.

## Correction Authority Envelope

- Observed violation: at exact HEAD `761f990b`, direct LLVM execution passed,
  but LCOV SHA `983b196b...` contained no
  `verifier_coverage_tests.rs` source record or owned function symbols; CRAP
  JSON SHA `391bf766...` therefore reported null coverage for all four CQR
  functions.
- In-scope source: `crates/openwepp-gate-planner/src/verifier.rs` module wiring.
- Authorized evidence surface:
  `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`, its direct
  characterization, and its existing real consumer test.
- Allowed edit: replace only the nested inline `include!` block with an
  equivalent path-backed private test module declaration.
- Acceptance: direct and consumer tests pass; focused LCOV contains the exact
  included-source path and attributable owned functions; CRAP coverage is
  non-null and every owned row is at most 30; formatting and planner Clippy pass;
  dual review accepts exact behavior/source identity.
- Protected boundaries: no production verifier logic, public API, fixture
  semantics, gate policy, threshold, or unrelated module organization changes.

## Conversion Rule

The root cause is reproduced, owned, safe, narrowly testable, and measurable
inside this envelope. The package must land and validate the direct wiring
correction; diagnostic-only HOLD is not permitted while that route remains
available.

## Intended Write Set

- `crates/openwepp-gate-planner/src/verifier.rs`
- `docs/work-packages/20260722-testgate-coverage-include-identity-recovery-001/**`
- `docs/work-packages/README.md`

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/defect_closure_execplans.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- this package
- the two authorized Rust files

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only implementation reviewers and two read-only
terminal verifiers. Expected outputs are package-local review/verification
artifacts. No HEAVY gate is selected for this bounded prerequisite.

## Plan

1. Retain the failing LCOV/CRAP identities and durable OPEN record.
2. Apply the one-seam path-backed module correction.
3. Run direct and consumer tests, focused LCOV/CRAP, formatting, and Clippy.
4. Obtain dual review, commit the correction, close RTR-043 against that exact
   commit, and complete dual verification.

## Exit Criteria

- RTR-043 has a durable CLOSED record bound to the reviewed correction commit.
- Source-attributable coverage and CRAP evidence pass.
- Dual implementation review and dual verification pass with no open finding.
