# OWCMP01 Comparator CLI Implementation

Status: queued
Created: 2026-06-08
Series: `owcmp` (comparison tooling)
Execution mode: package-end-to-end
Discipline: tooling implementation with contract-preserving migration from the
active PL14S legacy comparison suite.

## Objective

Implement `tools/owcmp` as the first-class openWEPP comparator CLI while
preserving the active PL14S WAT semantic comparator and replay-suite behavior
currently housed under `tools/legacy_comparison_suite`.

This package proves `owcmp` in parallel with the legacy suite. It must not delete
`tools/legacy_comparison_suite` or retarget all canonical references; that is
the follow-on cutover package.

## Rationale

`tools/legacy_comparison_suite` was intended to become a general comparator
surface but remained a PL14S-specific namespace. The HPHYS0298->0320 work showed
that repeated direct use of package-specific comparator scripts, raw logs, and
per-hillslope reports is context-expensive for Codex agents. `owcmp` provides a
stable command surface and compact summaries so execution-heavy comparison work
can be delegated without loading raw artifacts into the parent context.

## Included Scope

- Create the repo-local `tools/owcmp` CLI.
- Port or wrap the PL14S WAT semantic comparison behavior from
  `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`.
- Port or wrap the PL14S replay/strict/semantic suite behavior from
  `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`.
- Move or copy the PL14S tolerance config and Python dependency files into
  `tools/owcmp`, and prove byte-for-byte or effective-profile identity.
- Add compact summary generation for parent-agent-safe handoff.
- Add focused tests for the `owcmp` path without deleting or breaking the
  existing legacy-suite contract test.
- Update `tools/owcmp/specification.md` only as needed to keep implementation and
  contract wording aligned.
- Produce implementation, gate, review, verification, and handoff artifacts.

## Excluded Scope

- Deleting `tools/legacy_comparison_suite`.
- Retargeting all canonical docs/tests from `tools/legacy_comparison_suite` to
  `tools/owcmp`.
- Removing historical work-package artifact references to the legacy path.
- Implementing `owcmp observe normalize`.
- Changing comparator schemas, tolerance semantics, strict comparator authority,
  or ADR-0017 comparator posture.
- Any production kernel/runtime physics change.

## Deliverables

1. `tools/owcmp/` CLI and local documentation.
2. PL14S WAT semantic comparison command preserving
   `pl14s-semantic-wat-v2` output behavior.
3. PL14S replay/strict/semantic command preserving
   `pl14s-legacy-suite-v2` provenance behavior.
4. Compact `summary.json`/`summary.md` output for comparator reports.
5. Copied or moved PL14S tolerance config and Python dependency lock under
   `tools/owcmp`, with identity evidence.
6. Focused contract/regression tests for the new `owcmp` path.
7. Package artifacts: required-reading map, implementation evidence,
   tolerance-identity evidence, gate results, review/disposition, verification,
   worker handoff, and final disposition.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/codex_exec_plans.md`
- `tools/owcmp/specification.md`
- `tools/legacy_comparison_suite/README.md`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/subsystems/observability/legacy-observe-migration.md`

## Intended Write Set

- `tools/owcmp/**`
- `tests/integration/*owcmp*`
- `Cargo.toml` only if a new integration test target is required.
- `docs/work-packages/20260608-owcmp01-comparator-cli-implementation-001/**`

Allowed only if needed to keep local wording consistent:

- `tools/owcmp/specification.md`

Protected in this package:

- `tools/legacy_comparison_suite/**` remains present and runnable.
- Existing active canonical legacy-suite references remain for OWCMP02.

## Phase Plan

1. **Inventory and contract map.** Record the exact PL14S behavior and schema keys
   that must survive the port.
2. **CLI implementation.** Add `tools/owcmp/owcmp` and supporting modules/docs.
3. **Config/dependency migration.** Place tolerance config and Python dependency
   files under `tools/owcmp`; prove identity with legacy inputs.
4. **PL14S behavior preservation.** Run focused semantic and suite-level checks
   against small fixtures or existing contract fixtures.
5. **Compact summaries.** Implement summary generation and record sample output.
6. **Validation and review.** Run focused tests, dual reviews, disposition, dual
   verification, and final handoff.

## Acceptance Criteria

- `tools/owcmp/owcmp wat semantic` preserves the PL14S semantic report schema and
  required payload keys.
- `tools/owcmp/owcmp pl14s run` preserves strict-lane policy, candidate source
  classification, baseline-year policy, expected common-row-count handling,
  conversion row-consistency metadata, strict comparator hash capture, and
  `pl14s-legacy-suite-v2` provenance payload keys.
- `tools/owcmp/owcmp summarize` emits compact file-backed summaries suitable for
  parent-agent handoff.
- Tolerance config identity is proven against the legacy config.
- Focused `owcmp` tests pass.
- Existing legacy-suite contract tests are not weakened.
- No active code path relies on a long-lived silent compatibility wrapper.
- `owcmp observe normalize` remains deferred and unimplemented unless a separate
  observability package authorizes it.

## Required Gates

Focused iteration gates:

- `python3 -m py_compile` for new Python files.
- Focused `owcmp` CLI smoke/contract checks.
- Focused cargo test for any new integration test target.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract` to
  confirm the existing legacy-suite contract remains intact.

Pre-handoff sanity:

- `cargo fmt --check`
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`
- Any new `owcmp` integration test target.
- `git diff --check`

If broader workspace gates are skipped because this package is tooling-local,
record the rationale in `artifacts/gate-results.md`.

## Legitimate HOLD Conditions

- Existing PL14S behavior cannot be preserved without changing schema or
  tolerance semantics.
- The tolerance profile cannot be proven identical after relocation.
- The strict comparator authority or zero-tolerance invocation is unavailable.
- An implementation would require deleting or retargeting legacy-suite canonical
  references before OWCMP02.
- Observe normalization becomes necessary for PL14S cutover; route it to a
  separate observability work package instead of expanding this package.

## Review and Verification Requirements

- Dual independent reviews:
  `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`.
- Explicit finding disposition in `artifacts/review-disposition.md`.
- Dual verification:
  `artifacts/verification_agent_a.md` and
  `artifacts/verification_agent_b.md`.
- Final `artifacts/disposition.md` must state whether OWCMP02 cutover is ready
  and list any blockers.

## Autonomy

Execute package-end-to-end without asking for direction. Ask or HOLD only at a
declared boundary above. Do not expand scope into OWCMP02 cutover or observe
normalization.
