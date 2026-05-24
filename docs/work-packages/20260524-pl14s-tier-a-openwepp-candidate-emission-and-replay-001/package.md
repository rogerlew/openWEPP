# 20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001

## Status
- state: completed-with-hold
- date: 2026-05-24
- timezone: UTC

## Objective
Prepare and execute a semantic-parity Tier-A hillslope water-balance replay
between pinned legacy `wepp_260430_hill` and current openWEPP candidate outputs,
with dedicated investigation tooling that explains divergence causes.

## Why This Package Exists
PL15R reversal requires fresh `PL14S -> PL15S` closeout using provenance-valid
openWEPP-vs-legacy evidence. Current strict raw comparator outputs are useful
for structural checks but weak for diagnostics when candidate and baseline are
not similar.

PL14S must therefore do two things:
1. run the required Tier-A replay using openWEPP-emitted candidate outputs, and
2. produce reusable investigation-grade diagnostics for future legacy
   comparison cycles.

This package explicitly targets **semantic parity** for hillslope water
balance; erosion parity is out of scope until erosion kernels are promotable.

## Scope
### Included
- Establish PL14S replay lane authority and guard posture for semantic
  hillslope water-balance comparison.
- Wire dedicated reusable legacy-comparison suite tooling for:
  - baseline replay provenance capture,
  - strict raw compare where format-compatible,
  - semantic row/column delta diagnostics,
  - investigation bundle output with top divergent keys.
- Execute Tier-A replay with openWEPP candidate outputs and capture artifacts.
- Record binary/tool/output hashes and command traces for reproducibility.
- Preserve existing typed seam and confidence-tier routing non-regression.

### Explicitly Out of Scope
- Erosion/sediment parity closeout.
- Watershed/hourly comparator promotion gates.
- Final PL08 hold-lift disposition (`PL15S` lane).

## Deliverables
1. PL14S contract implementation evidence:
   - `artifacts/pl14s-contract-implementation-evidence.md`
2. PL14S replay lane configuration and guard map:
   - `artifacts/pl14s-replay-lane-configuration-and-guard-map.md`
3. PL14S contract-derived test implementation evidence:
   - `artifacts/pl14s-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl14s-preimplementation-contract-gate.md`
5. Comparator run provenance manifest:
   - `artifacts/pl14s-comparator-run-provenance-manifest.md`
6. Tier-A semantic comparator delta report:
   - `artifacts/pl14s-tier-a-semantic-parity-delta-report.md`
7. Comparator JSON artifact index:
   - `artifacts/pl14s-comparator-json-artifact-index.md`
8. Legacy comparison suite design note:
   - `artifacts/pl14s-legacy-comparison-suite-design.md`
9. Legacy comparison investigation playbook:
   - `artifacts/pl14s-legacy-comparison-suite-investigation-playbook.md`
10. Execution and test evidence:
   - `artifacts/pl14s-implementation-and-test-evidence.md`
11. Kernel-profile/runtime-contract compliance checklist:
   - `artifacts/pl14s-kernel-profile-compliance-checklist.md`
12. Erosion exclusion scope note:
   - `artifacts/pl14s-erosion-exclusion-note.md`
13. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl14s_disposition.md`
14. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
15. Persisted comparator artifacts:
   - `artifacts/h5_wat_strict_comparator.json`
   - `artifacts/h5_wat_semantic_comparator.json`
   - `artifacts/pl14s_provenance_manifest.json`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0003-parity-semantic-not-bit.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/numerics/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/**`
- `tools/legacy_comparison_suite/**`
- `docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL14S queue authority and dependency closure state.
- Confirm pinned baseline binary/tool availability and fixture lane inputs.

### Phase A - Contract/Tooling Authority
- Ratify semantic-parity replay posture and investigation bundle requirements.
- Align PL14S guard posture with confidence-tier and semantic-parity policy.

### Phase B - Contract Tests + Pre-Implementation Gate
- Implement contract-derived PL14S tests for replay lane and semantic report
  requirements.
- Record pre-implementation contract gate before replay/harness code edits.

### Phase C - Replay Execution + Investigation Bundle
- Execute baseline replay and openWEPP candidate compare.
- Persist strict and semantic comparator reports with provenance hashes.
- Produce divergence investigation outputs for non-similar trajectories.

### Phase D - Verification + Disposition
- Run required repository gates if code changed.
- Publish dual review/verification and PL14S disposition for PL15S use.

## Exit Criteria
- PL14S replay executes with provenance-valid openWEPP-vs-legacy evidence.
- Hillslope water-balance semantic comparison report is generated with
  per-column diagnostics and top divergent rows.
- Investigation bundle includes command trace, hashes, row-key presence deltas,
  and tolerance verdicts.
- Erosion surfaces are explicitly excluded and do not block PL14S scope.
- Required comparator artifacts are persisted under package artifacts.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: replay/comparator tooling and evidence package for parity
  assessment; no production secret or external service mutation paths.
