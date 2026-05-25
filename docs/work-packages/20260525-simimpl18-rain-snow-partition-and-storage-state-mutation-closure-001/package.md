# 20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-25
- timezone: UTC

## Objective
Close the SIMIMPL17 hydrologic semantic blockers by restoring day-1 rain/snow
partition parity, removing static-parameter-to-state publication leakage, and
restoring dynamic winter/soil storage-state mutation under identical
candidate/baseline inputs and sidecars, including full-span precipitation
parity over all 1095 keyed records.

## Why This Package Exists
SIMIMPL17 retained `HOLD` with concrete hydrologic clues that point to two
physics-critical failures under shared inputs:
1. first shared key mismatch (`OFE=1/J=1/Y=1`) where baseline routes `P=4.4`
   into snow-water (`RM=0.00`, `Snow-Water=4.40`) while candidate routes to
   effective melt/rain (`RM=4.40`, `Snow-Water=250.00`),
2. candidate storage surfaces remain invariant for all 1095 rows
   (`Total-Soil=76.00`, `frozwt=0.00`, `Snow-Water=250.00`,
   `SoilWaterTotal=76.00`),
3. candidate provenance records
   `coupling_vectors.winter.ssd=250.0` and
   `coupling_vectors.hydout_equivalent.snow_water=250.0`, consistent with a
   static-parameter-to-state publication leak in winter/hydout mapping,
4. baseline lane logs report one-year clamp warnings
   (`Number of years to simulate can't be larger than 1`; `1 years used`),
   preventing full-span 1095-row parity closure.

This package is the follow-on closure wave to resolve those two defects using
contract-first sequencing and replay evidence.

## Scope
### Included
- Amend canonical science contracts for day-1 rain/snow partition expectations,
  winter-state evolution, and published storage-state mutation invariants.
- Add contract-derived tests that fail on SIMIMPL17 behavior and pass after
  implementation.
- Record pre-implementation contract gate evidence before production edits.
- Implement production fixes for:
  - rain/snow partition coupling at runtime,
  - winter/hydout publication mapping to prevent static sidecar parameter
    leakage (for example `ssd`) into dynamic `Snow-Water` state outputs,
  - hydout/water-balance publication mapping so dynamic state is emitted rather
    than static sidecar parameters,
  - state progression across day-indexed execution.
- Implement explicit baseline-year policy handling for replay lanes so baseline
  and candidate are compared over the same 1095-day horizon for this fixture.
- Add/upgrade replay tooling assertions so precipitation (`P`) parity is
  evaluated across all 1095 keyed rows, not only overlap-clamped subsets.
- Re-run Tier-A replay lanes with identical candidate/baseline input files and
  sidecars, then evaluate hold-lift criteria impact.
- Publish disposition and residual ownership.

### Explicitly Out of Scope
- Watershed channel/impoundment physics kernel changes not required by these
  hillslope hydrology closures.
- Non-hydrology replay feature expansion unrelated to identified blockers.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl18-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl18-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl18-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/simimpl18-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl18-kernel-profile-compliance-checklist.md`
6. Focus diagnostics:
   - `artifacts/simimpl18-first-day-rain-snow-partition-diagnostic.md`
   - `artifacts/simimpl18-storage-state-mutation-diagnostic.md`
   - `artifacts/simimpl18-winter-publication-leak-diagnostic.md`
   - `artifacts/simimpl18-baseline-year-policy-and-precip-span-closure.md`
7. Replay rerun and criteria evaluation:
   - `artifacts/simimpl18-tier-a-rerun-execution-evidence-index.md`
   - `artifacts/simimpl18-closure-criteria-evaluation-matrix.md`
8. Final decision artifacts:
   - `artifacts/simimpl18-go-no-go-verdict.md`
   - `artifacts/simimpl18-final-disposition-decision-memo.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl18_disposition.md`
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
This package must remain self-contained and executable end-to-end. Assigned
agents must progress through all declared phases and update artifacts through
final disposition without requesting additional user direction unless
hard-blocked by missing local authority, unreadable dependencies, or
contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:`
and/or `Ran:` sections. Claims without explicit evidence-mode labeling are
non-compliant.

## Provenance and Authority Posture
- Canonical science authority is `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and cannot replace canonical authority.
- Legacy baseline provenance defaults to:
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Parity reruns must use identical soil, landuse/management, slope, climate,
  and sidecar files for baseline and candidate lanes.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-replay-parity-full-closure-criteria.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/replay-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/simimpl17_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/simimpl17-residual-classification-and-hold-lift-rationale.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/comparator_tier_routing_metadata.rs`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Baseline Signal Freeze
- Confirm authorization from SIMIMPL17 residual/disposition artifacts.
- Freeze the exact first-day mismatch, invariant-state evidence set, winter
  provenance leak signal (`ssd` vs published `Snow-Water`), and baseline
  one-year clamp warnings as package entry signals.

### Phase B - Canonical Contract Amendments
- Amend canonical `SC-*` contracts to codify:
  - day-1 rain/snow partition expectations under cold conditions,
  - dynamic mutation expectations for `Snow-Water`, `frozwt`, `Total-Soil`,
    and `SoilWaterTotal`,
  - publication mapping constraints between runtime state and WB13 output,
  - prohibition on static sidecar parameter publication as dynamic storage
    state outputs,
  - baseline/candidate replay-span policy requirements for parity evaluation.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate
- Implement/extend replay and integration tests from amended authority.
- Include explicit tests for:
  - first-day rain/snow partition parity under shared inputs,
  - non-invariant winter/storage state mutation across multi-day forcing,
  - precipitation parity across 1095 keyed rows under equal baseline/candidate
    run span,
  - provenance separation between static winter parameters and dynamic emitted
    state.
- Record pre-implementation contract gate evidence before production edits.

### Phase D - Production Fixes and Tier-A Rerun
- Implement hydrology/runtime/publication fixes in runner/CLI surfaces.
- Re-run candidate and baseline parity lanes with identical inputs and sidecars
  and explicit baseline-year policy closure for 1095-day comparison span.
- Persist rerun logs/manifests/comparator outputs.

### Phase E - Evaluation and Disposition
- Re-score SIMIMPL13 criteria impacted by this scope.
- Publish `GO`/`HOLD` verdict with residual ownership and explicit next steps.
- Complete dual review and dual verification artifacts.

## Exit Criteria
- First shared key (`OFE=1/J=1/Y=1`) parity for targeted columns is explained by
  contract-compliant behavior and validated by tests/evidence.
- Candidate winter/soil storage surfaces are no longer invariant across a
  varying-forcing multi-day run unless canonical authority explicitly allows it.
- Provenance and emitted state no longer show static-parameter leakage where
  `coupling_vectors.hydout_equivalent.snow_water` mirrors static
  `coupling_vectors.winter.ssd` absent runtime justification.
- Baseline lane no longer enforces one-year clamp for this replay fixture and
  produces the same 1095-row keyed span as candidate for parity comparison.
- Precipitation series (`P`) parity is demonstrated across all 1095 keyed rows
  for baseline vs candidate under identical input and sidecar files.
- Contract-derived tests fail on pre-fix behavior and pass on final behavior.
- Tier-A rerun bundle is reproducible with command/log/provenance traces.
- Required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Contract-first sequence evidence is complete:
  1. contract implementation,
  2. contract-test implementation,
  3. pre-implementation contract gate,
  4. production implementation and rerun evidence.
- Dual review and dual verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: local hydrology/runtime parity closure and evidence workflow; no
  external privileged integration changes.
