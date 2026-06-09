# OWCMP01 Kickoff — Comparator CLI Implementation

Scope: local repository tooling implementation task; flat-file edits and local
test execution only; no production operations.
Execution mode: package-end-to-end.

Autonomy: execute end-to-end — implement `tools/owcmp`, preserve PL14S semantic
and replay-suite behavior, add compact summaries and focused tests, record gate
evidence through disposition — without asking for direction. Ask or HOLD only at
the hard stops below.

## What and why

`tools/legacy_comparison_suite` is active PL14S comparator infrastructure but
the name and direct-script workflow are now a long-term liability. Implement
`tools/owcmp` as the first-class comparator CLI while proving behavior in
parallel. This package does not delete or fully retarget the legacy suite; that
is OWCMP02.

## Required reading

Core:

- `package.md`
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `tools/owcmp/specification.md`

Conditional:

- `tools/legacy_comparison_suite/README.md`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

On-demand:

- `docs/specifications/subsystems/observability/legacy-observe-migration.md`
  (only if observe-sidecar questions arise; do not implement observe normalize)
- `docs/codex_exec_plans.md`

Record reading status in `artifacts/required-reading-map.md`.

## Tasks

1. Inventory current PL14S semantic and suite runner behavior, including schema
   keys, strict-lane policy, baseline-year policy, conversion row-consistency,
   and tolerance defaults. Record in `artifacts/implementation-test-evidence.md`.
2. Implement `tools/owcmp/owcmp` with:
   - `wat semantic`
   - `pl14s run`
   - `summarize`
   - `manifest run` if feasible for the PL14S lane
3. Move or copy PL14S tolerance config and Python dependency files under
   `tools/owcmp`; prove identity in `artifacts/tolerance-identity.md`.
4. Add focused `owcmp` tests without weakening the existing
   `pl14s_tier_a_candidate_emission_and_replay_contract`.
5. Run focused gates and record command-level evidence in
   `artifacts/gate-results.md`.
6. Complete dual review, finding disposition, dual verification, final
   disposition, and worker handoff.

## Outputs

- `tools/owcmp` CLI, support files, and docs.
- Focused tests for the `owcmp` path.
- `artifacts/`: required-reading map, implementation/test evidence,
  tolerance-identity evidence, gate results, dual review, review disposition,
  dual verification, final disposition, and worker handoff.

## Hard stops

- Preserving PL14S behavior requires schema/tolerance changes.
- Strict comparator authority
  `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py` is
  unavailable or cannot be hashed.
- Tolerance config identity cannot be proven.
- A requested edit would delete `tools/legacy_comparison_suite` or retarget all
  canonical references before OWCMP02.
- Observe normalization appears necessary; HOLD and route to a separate
  observability package.
