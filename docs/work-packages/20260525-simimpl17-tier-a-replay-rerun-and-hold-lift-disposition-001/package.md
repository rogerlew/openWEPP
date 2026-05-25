# 20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001

## Status
- state: package-complete-with-hold
- date: 2026-05-25
- timezone: UTC

## Objective
Execute full Tier-A replay reruns (strict + semantic, dat + parquet lanes as
applicable) after SIMIMPL14/15/16 closure, evaluate results against SIMIMPL13
closure criteria, and publish final hold-lift disposition (`GO`/`HOLD`) with
explicit blocker ownership when criteria remain unmet.

## Why This Package Exists
SIMIMPL13 defined final promotability criteria (`SIMIMPL13-CRIT-001..008`) and
queued SIMIMPL17 as the disposition wave after implementation/test closures.
SIMIMPL14, SIMIMPL15, and SIMIMPL16 are now complete with downstream `GO`
verdicts, enabling an authoritative rerun and final decision pass.

This package is the decision gateway: it must produce reproducible replay
artifacts and an evidence-backed final posture for hillslope replay/parity
promotion.

## Scope
### Included
- Verify prerequisite package completion state and dependency readability.
- Implement/ratify any required canonical contract amendments for rerun gate
  interpretation and disposition semantics.
- Implement contract-derived rerun/disposition tests if required by updated
  authority before executing final reruns.
- Execute strict and semantic replay workflows using current runner/tooling
  surfaces and collect provenance-complete evidence bundles.
- Evaluate rerun outputs against `SIMIMPL13-CRIT-001..008` and publish final
  disposition (`GO` or retained `HOLD`) with explicit residual classification.

### Explicitly Out of Scope
- New core runner continuous-execution implementation (SIMIMPL14 scope).
- New comparator production alignment implementation (SIMIMPL15 scope).
- New blind-spot closure test implementation (SIMIMPL16 scope).

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl17-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl17-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl17-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/simimpl17-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl17-kernel-profile-compliance-checklist.md`
6. Tier-A rerun execution evidence index:
   - `artifacts/simimpl17-tier-a-rerun-execution-evidence-index.md`
7. Closure-criteria evaluation matrix:
   - `artifacts/simimpl17-closure-criteria-evaluation-matrix.md`
8. Residual classification and hold-lift rationale:
   - `artifacts/simimpl17-residual-classification-and-hold-lift-rationale.md`
9. Final disposition decision memo:
   - `artifacts/simimpl17-final-disposition-decision-memo.md`
10. Governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/simimpl17_disposition.md`
11. Dual review artifacts:
    - `artifacts/review_agent_a.md`
    - `artifacts/review_agent_b.md`
12. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract updates in `SC-*` files (if required).
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then execute replay rerun/disposition workflow and associated tooling
   edits if still required.

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
- Canonical replay/parity closure authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md` and SIMIMPL13
  closure criteria surfaces.
- Legacy baseline provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Disposition claims must map directly to measurable criteria and rerun
  artifacts; no inferred promotion without explicit evidence.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-replay-parity-full-closure-criteria.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/replay-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/artifacts/simimpl14-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/artifacts/simimpl14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/artifacts/simimpl15-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/artifacts/simimpl15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl16-replay-contract-derived-test-coverage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl16-replay-contract-derived-test-coverage-closure-001/artifacts/simimpl16-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl16-replay-contract-derived-test-coverage-closure-001/artifacts/simimpl16_disposition.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/comparator_tier_routing_metadata.rs`
- `docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Entry Confirmation
- Confirm SIMIMPL17 queue authorization and prerequisite completion
  (`SIMIMPL14`, `SIMIMPL15`, `SIMIMPL16` `GO`).
- Confirm dependency readability and rerun environment readiness.

### Phase B - Canonical Contract and Gate Ratification
- Implement/ratify required canonical `SC-*` amendments for rerun gate
  interpretation and final disposition semantics.
- Update contract index cross-links when authority surfaces change.

### Phase C - Contract-Derived Rerun/Disposition Gate
- Implement contract-derived rerun/disposition tests if required by authority
  updates.
- Execute and record pre-implementation contract gate evidence.

### Phase D - Tier-A Replay Rerun Execution
- Execute strict + semantic replay lanes (dat/parquet as policy permits).
- Collect provenance-complete rerun artifacts, manifests, and comparator
  outputs.

### Phase E - Closure Evaluation and Final Disposition
- Evaluate rerun outputs against `SIMIMPL13-CRIT-001..008`.
- Publish final `GO`/`HOLD` disposition with explicit blocker ownership and
  dual review/verification evidence.

## Exit Criteria
- Rerun artifact bundle is complete and reproducible for all required lanes
  with command/provenance traceability.
- `SIMIMPL13-CRIT-001..007` are evaluated explicitly against rerun evidence and
  each marked `pass`/`fail` with citation.
- `SIMIMPL13-CRIT-008` governance completeness is evaluated and risk posture
  recorded per SIMIMPL13 hold policy.
- Final disposition is explicit (`GO` or retained `HOLD`) with concrete
  residual ownership if `HOLD` remains.
- Required repository gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Contract-first sequence evidence is complete:
  1. contract implementation,
  2. contract-test implementation,
  3. pre-implementation contract gate,
  4. rerun/disposition implementation evidence.
- Dual review/disposition/verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: replay rerun/disposition and evidence packaging scope; no external
  privileged integrations are required.
