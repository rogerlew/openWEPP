# 20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-25
- timezone: UTC
- decision: HOLD

## Objective
Implement baseline-authoritative closure for `RM` and `Snow-Water` publication
under identical legacy/candidate inputs by migrating the required legacy
rain/snow partition and winter-state publication behavior into openWEPP
architecture without heuristic substitutions.

## Why This Package Exists
SIMIMPL18 closed with `HOLD` after replay-span tooling closure, but retained
physics-critical residuals under shared fixture inputs:
1. day-1 partition mismatch (`P=4.4` with baseline `RM=0.0`, candidate
   `RM=4.4`),
2. day-1 winter-state mismatch (baseline `Snow-Water=4.4`, candidate
   `Snow-Water=250.0`),
3. static publication leak where candidate emitted `Snow-Water` mirrors
   `snow.options.ssd` instead of runtime SWE,
4. storage tuple closure remains unresolved and workspace tests remain failing
   for SIMIMPL18 contract-derived checks.

This package is the follow-on implementation wave focused on process-physics
closure for `RM` and `Snow-Water` with strict baseline-authoritative
provenance and no proxy equations.

## Scope
### Included
- Amend canonical science contracts for:
  - rain/snow partition semantics that govern published `RM`,
  - runtime SWE state progression and publication semantics that govern
    published `Snow-Water`,
  - explicit prohibition on publishing static winter control parameters as
    dynamic output state,
  - guard/typed-error behavior for required winter coupling state.
- Build a provenance map from `/workdir/wepp-forest_260430_baseline` showing
  exact routine-level authority for `RM` and `Snow-Water` behavior used in
  migration.
- Add contract-derived tests that fail against current SIMIMPL18 residual
  behavior and pass only after baseline-authoritative migration.
- Record pre-implementation contract-gate evidence before production edits.
- Implement production runtime/publication changes in openWEPP to:
  - publish `RM` from contract-authoritative rain/melt partition behavior,
  - publish `Snow-Water` from runtime SWE state (not static sidecar controls),
  - preserve typed guard posture (no silent defaults/clamping),
  - preserve identical-input parity lane assumptions.
- Rerun Tier-A replay lanes with identical soil/landuse/slope/climate/sidecar
  files for baseline and candidate and publish refreshed diagnostics/disposition.

### Explicitly Out of Scope
- Unrelated watershed channel/impoundment kernel work.
- Broad ET/percolation refactors not required by RM/Snow-Water closure scope.
- Any heuristic/provisional process-physics substitutions.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl19-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl19-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl19-preimplementation-contract-gate.md`
4. Implementation and test evidence:
   - `artifacts/simimpl19-implementation-and-test-evidence.md`
5. Baseline authority/provenance map:
   - `artifacts/simimpl19-rm-snow-water-baseline-provenance-map.md`
6. Focus diagnostics:
   - `artifacts/simimpl19-first-day-rain-snow-partition-diagnostic.md`
   - `artifacts/simimpl19-runtime-swe-publication-diagnostic.md`
   - `artifacts/simimpl19-storage-state-mutation-diagnostic.md`
7. Replay rerun and closure evaluation:
   - `artifacts/simimpl19-tier-a-rerun-evidence-index.md`
   - `artifacts/simimpl19-closure-criteria-evaluation-matrix.md`
8. Final decision artifacts:
   - `artifacts/simimpl19-go-no-go-verdict.md`
   - `artifacts/simimpl19-final-disposition-decision-memo.md`
9. Kernel profile and governance artifacts:
   - `artifacts/simimpl19-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl19_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
    - `artifacts/review_agent_a.md`
    - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract updates in `SC-*` files.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then modify production code and execute reruns.

Any sequencing violation keeps package disposition in `HOLD`.

## Autonomous Execution Intent (Required)
This package must be executable end-to-end without user intervention. Assigned
agents must execute all phases through disposition and update required artifacts
without requesting additional direction unless hard-blocked by missing local
authority or contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Claims without evidence-mode labeling are non-compliant.

## Provenance and Authority Posture
- Canonical authority is in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- No heuristic, surrogate, or proxy process-physics equations are permitted for
  touched RM/Snow-Water pathways.
- If ET-related publication behavior must be touched to preserve closure, use
  baseline-authoritative evap routines rather than placeholder formulas.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18-first-day-rain-snow-partition-diagnostic.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18-winter-publication-leak-diagnostic.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18-closure-criteria-evaluation-matrix.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` (only if
  required by RM/Snow-Water closure semantics)
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/lib.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py` (if replay harness
  policy updates are required)
- `docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Residual Freeze
- Freeze SIMIMPL18 residual signals and exact failing assertions for RM and
  Snow-Water.
- Freeze identical-input parity constraints for candidate/baseline reruns.

### Phase B - Canonical Contract Authority Update
- Amend canonical `SC-*` authority for RM partition, runtime SWE publication,
  and static-parameter leak prohibition.
- Record baseline routine-level provenance mapping for each touched invariant.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate
- Implement tests derived from updated contracts.
- Capture pre-implementation gate evidence before any production edits.

### Phase D - Production Migration and Tier-A Rerun
- Implement baseline-authoritative RM/Snow-Water runtime/publication behavior.
- Run required gates and Tier-A reruns under identical inputs.
- Persist manifests, comparator outputs, and diagnostics.

### Phase E - Evaluation and Disposition
- Evaluate closure criteria and publish GO/HOLD decision with residual
  ownership.
- Complete dual review and dual verification artifacts.

## Exit Criteria
- Day-1 parity closure for targeted fields at shared key (`OFE=1/J=1/Y=1`):
  - `RM`: baseline-aligned under cold all-snow forcing (expected `0.0` in
    SIMIMPL18 fixture),
  - `Snow-Water`: runtime SWE-aligned under day-1 forcing (expected `4.4` in
    SIMIMPL18 fixture).
- Candidate no longer publishes static `snow.options.ssd` as dynamic
  `Snow-Water` output in RM/Snow-Water closure lane.
- Contract-derived SIMIMPL18/SIMIMPL19 RM/Snow-Water assertions pass.
- Storage tuple diagnostics reflect dynamic mutation consistent with updated
  contract authority.
- Tier-A replay evidence refreshed using identical baseline/candidate inputs and
  sidecars.
- Required gates executed and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Contract-first sequence evidence complete and auditable.
- Dual review and dual verification artifacts completed.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: local process-physics/state-publication migration and replay
  verification only.
