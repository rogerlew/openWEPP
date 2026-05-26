# 20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001

## Status
- state: package-complete
- date: 2026-05-26
- timezone: UTC
- decision: GO

## Objective
Execute SIMIMPL36 by closing SIMIMPL35 frost hold blockers, rerunning fresh
post-SIMIMPL34 candidate lanes, and publishing an explicit GO/HOLD disposition
for winter-hourly frost parity readiness.

## Why This Package Exists
SIMIMPL35 completed rerun/disposition scope with `HOLD` due to three blockers:
1. shared-fixture typed domain failure (`KWRITEBACK-E-DOMAIN-VIOLATION`),
2. `/wc1` parser compatibility failure (`SOL-E-006` on quoted legacy header),
3. unfiltered parquet comparator non-admissibility from duplicate `(OFE,J,Y)` keys.

SIMIMPL36 is the authorized follow-on closure package for those blockers.

## Scope
### Included
- Contract-first authority amendments for touched canonical contracts.
- Contract-derived test implementation for soil compatibility and WB14 runoff
  normalization behavior.
- Pre-implementation contract gate capture.
- Production/runtime/tooling changes needed to close all three SIMIMPL35
  blockers.
- Fresh post-fix reruns and comparator evidence publication.
- Required gates, dual review, dual verification, and final disposition.

### Explicitly Out of Scope
- New frost process-physics beyond SIMIMPL35 blocker closure scope.
- Non-frost/non-WB12 parser/kernel rewrites unrelated to blocker closure.
- Branching or repository-history rewrites.

## Deliverables
1. `artifacts/simimpl36-winter-hourly-semantic-parity-evidence-report.md`
2. `artifacts/simimpl36-hold-lift-decision-report.md`
3. `artifacts/simimpl36-contract-implementation-evidence.md`
4. `artifacts/simimpl36-contract-test-implementation-evidence.md`
5. `artifacts/simimpl36-preimplementation-contract-gate.md`
6. `artifacts/simimpl36-implementation-and-test-evidence.md`
7. `artifacts/simimpl36-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/simimpl36_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting work in SIMIMPL36 follows this order:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code and runtime tooling.

## Autonomous Execution Intent (Required)
This package is execution-ready and must run end-to-end through disposition
without user intervention unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must label claim class explicitly using `Static:` and
`Ran:` sections.

## Physics and Provenance Authority
- Baseline-authoritative comparator and migration reference:
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions in production paths.
- Canonical authority remains in `SC-*` contracts.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/artifacts/simimpl35-hold-lift-decision-report.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`

## Intended Write Set
- `docs/work-packages/20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-input-contract/src/parsers/soil.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/infile_soil_parser_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/fixtures/infile/soil/compat_quoted_header_9002_policy_first.sol`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/README.md`

## Phase Plan
### Phase A - Intake and Authorization Confirmation
- Confirm SIMIMPL35 hold evidence and blocker ownership.
- Confirm SIMIMPL36 scope and dependency authority.

### Phase B - Contract/Test/Gate Prerequisites
- Apply required canonical contract amendments.
- Apply contract-derived tests.
- Record pre-implementation contract gate evidence.

### Phase C - Implementation and Runtime/Tooling Closure
- Implement parser/runtime/tooling fixes for all three blockers.
- Re-run candidate lanes and comparator lanes with fresh outputs.

### Phase D - Required Gates and Governance Artifacts
- Run required validation gates and capture outputs.
- Complete governance artifacts, dual review, and dual verification.

### Phase E - Disposition
- Publish explicit GO/HOLD decision with residual ownership and next-step
  handoff.

## Exit Criteria
- Shared-fixture candidate lane no longer fails with
  `KWRITEBACK-E-DOMAIN-VIOLATION` in SIMIMPL35 replay context.
- Direct `/wc1` candidate lane no longer fails with `SOL-E-006` for quoted
  legacy policy-first soil headers.
- `/wc1` semantic comparator lane is admissible via explicit partitioned
  parquet policy and records non-zero overlap.
- Required gates are executed and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- All required artifacts are complete and truthfully labeled.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- rationale: kernel/runtime/parser/comparator closure package with typed-guard
  and provenance-sensitive behavior updates; no new external interfaces.

## Execution Outcome Summary
- Shared-fixture rerun now succeeds with no
  `KWRITEBACK-E-DOMAIN-VIOLATION`.
- Direct `/wc1` rerun now succeeds with no `SOL-E-006` parser blocker.
- Fresh `/wc1` semantic lane executes with `common_row_count=1095`,
  `only_baseline_count=0`, `only_candidate_count=0` using explicit
  candidate year-key offset support for simulation-year keyed candidate rows.
- Required gates pass in
  `artifacts/gates-20260526T170356Z/`.
