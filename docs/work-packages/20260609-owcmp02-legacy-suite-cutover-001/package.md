# OWCMP02 Legacy Suite Cutover

Status: complete
Created: 2026-06-09
Series: `owcmp` (comparison tooling)
Execution mode: package-end-to-end
Discipline: tooling cutover and dead-path removal after OWCMP01 implementation.

## Objective

Cut active PL14S comparison tooling references over from
`tools/legacy_comparison_suite` to `tools/owcmp`, then remove
`tools/legacy_comparison_suite` from the repository.

## Rationale

OWCMP01 implemented the first-class `tools/owcmp` comparator CLI while keeping
the legacy suite in parallel. OWCMP02 completes the intended migration so the
repo does not carry two active comparator namespaces.

## Included Scope

- Retarget active docs/tests from `tools/legacy_comparison_suite` to
  `tools/owcmp`.
- Update the root README Python-tooling setup to use `tools/owcmp`.
- Update active science-contract tolerance-path references to `tools/owcmp`.
- Update the PL14S contract test to bind to `tools/owcmp`.
- Correct active `tools/owcmp` docs and generated lock comments that still name
  the legacy suite.
- Delete `tools/legacy_comparison_suite`.
- Record remaining `legacy_comparison_suite` hits and classify them as
  historical artifacts or blockers.
- Produce gate, review, verification, handoff, and final disposition artifacts.

## Excluded Scope

- Rewriting historical work-package artifacts that record commands or evidence
  from past runs.
- Implementing full manifest schema/identity/promotability validation.
- Implementing `owcmp observe normalize`.
- Changing PL14S schemas, tolerance values, strict comparator authority, or
  ADR-0017 comparator posture.
- Any production kernel/runtime physics change.

## Deliverables

1. Active reference cutover to `tools/owcmp`.
2. Removed `tools/legacy_comparison_suite`.
3. Passing focused PL14S and `owcmp` contract tests.
4. `rg legacy_comparison_suite` disposition showing no active blockers.
5. Package artifacts: required-reading map, implementation evidence,
   gate results, review/disposition, verification, worker handoff, and final
   disposition.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `tools/owcmp/specification.md`
- `tools/owcmp/README.md`
- `docs/work-packages/20260608-owcmp01-comparator-cli-implementation-001/artifacts/disposition.md`
- `docs/work-packages/20260608-owcmp01-comparator-cli-implementation-001/artifacts/worker-handoff.md`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/owcmp_cli_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `README.md`

## Intended Write Set

- `README.md`
- `tools/owcmp/**`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260609-owcmp02-legacy-suite-cutover-001/**`
- deletion of `tools/legacy_comparison_suite/**`

Allowed if needed to keep cutover wording aligned:

- `tools/owcmp/specification.md`

Protected:

- Historical work-package artifacts that merely record prior legacy-suite
  commands/evidence may keep their old paths.
- No production Rust implementation paths may be changed except the focused
  integration test.

## Phase Plan

1. **Reference inventory.** Separate active references from historical evidence.
2. **Package scaffold.** Create OWCMP02 artifacts and cutover authority.
3. **Active cutover.** Retarget README, SC-SYSTEM, active tests, and `owcmp`
   docs/comments.
4. **Legacy deletion.** Delete `tools/legacy_comparison_suite`.
5. **Validation.** Run focused `owcmp` and PL14S contract tests plus path checks.
6. **Review and closure.** Complete dual reviews, disposition, dual
   verification, worker handoff, and final disposition.

## Acceptance Criteria

- `tools/legacy_comparison_suite` no longer exists.
- Active tests/docs bind to `tools/owcmp`.
- `cargo test --test owcmp_cli_contract` passes.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`
  passes.
- `python3 -m py_compile` passes for `tools/owcmp` Python files.
- `cargo fmt --check` passes.
- `git diff --check` passes.
- `rg legacy_comparison_suite` returns only historical work-package artifacts
  and OWCMP package/spec references that intentionally document the migration,
  or returns no hits.
- No `__pycache__` artifacts remain.

## Required Gates

- `python3 -m py_compile tools/owcmp/semantic_wat.py tools/owcmp/pl14s_suite.py tools/owcmp/summary.py tools/owcmp/owcmp`
- `cargo test --test owcmp_cli_contract`
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`
- `cargo fmt --check`
- `git diff --check`
- `test ! -e tools/legacy_comparison_suite`
- `find tools/owcmp tests/integration docs/work-packages/20260609-owcmp02-legacy-suite-cutover-001 -type d -name __pycache__ -print`
- `rg legacy_comparison_suite`

## Legitimate HOLD Conditions

- Retargeted PL14S contract behavior cannot pass through `tools/owcmp`.
- Active non-historical references require the legacy directory to remain.
- Deleting the legacy directory would require changing PL14S schemas, tolerance
  values, strict comparator authority, or observe-normalization scope.

## Review and Verification Requirements

- Dual independent reviews:
  `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`.
- Explicit finding disposition in `artifacts/review-disposition.md`.
- Dual verification:
  `artifacts/verification_agent_a.md` and
  `artifacts/verification_agent_b.md`.
- Final `artifacts/disposition.md` must state whether the legacy suite was
  removed and whether any active blockers remain.

## Autonomy

Execute package-end-to-end without asking for direction. Ask or HOLD only at a
declared boundary above.

## Outcome

Completed on 2026-06-09.

- Retargeted active README, SC-SYSTEM tolerance, PL14S integration test, and
  `owcmp` docs/lock comments to `tools/owcmp`.
- Deleted `tools/legacy_comparison_suite`.
- Preserved historical work-package artifact references as archival evidence.
- Ran all required OWCMP02 gates successfully; see `artifacts/gate-results.md`.
