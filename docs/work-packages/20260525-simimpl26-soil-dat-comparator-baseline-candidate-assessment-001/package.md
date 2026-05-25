# 20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Execute a reproducible baseline-vs-candidate comparison workflow for `soil.dat`
(and canonical `.sol` soil input equivalents), publish authoritative delta
classification evidence, and produce explicit follow-on disposition guidance.

## Why This Package Exists
SIMIMPL25 closed WB11 Tier-A rerun and hold-lift posture for water-balance
surfaces. A user-directed follow-on request now requires explicit soil-file
comparison evidence so unresolved soil-input or soil-profile delta risk can be
assessed with provenance-backed findings.

## Scope
### Included
- Discovery and evidence capture for baseline/candidate soil-file artifacts
  referred to as `soil.dat` and/or `.sol` in active replay lanes.
- Reproducible provenance recording for compared file paths, hashes,
  line/field counts, and parser-version markers (`datver`/header forms).
- Deterministic delta classification for structural, numeric, and semantic
  differences across compared soil files.
- Governance artifacts, gate evidence, dual review/verification, and explicit
  disposition with follow-on recommendations.

### Explicitly Out of Scope
- New production kernel/process-physics edits unless comparison findings
  conclusively require corrective follow-on work.
- Silent normalization/clamping of soil-domain differences.
- Unrelated observability/docs/planning modifications outside SIMIMPL26 write
  scope.

## Deliverables
1. Soil-file comparison report:
   - `artifacts/simimpl26-soil-dat-comparison-report.md`
2. Soil-file provenance manifest:
   - `artifacts/simimpl26-soil-dat-provenance-manifest.md`
3. Contract implementation evidence:
   - `artifacts/simimpl26-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl26-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl26-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl26-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl26-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl26_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory when corrective implementation is
required:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL26 is expected to be comparison/disposition-heavy. If findings require
production edits, this sequence is mandatory before code changes.

## Autonomous Execution Intent (Required)
This package must be executable end-to-end without user intervention. Assigned
agents must execute all phases through disposition and update required artifacts
without requesting additional direction unless hard-blocked by contradictory
canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Claims without evidence-mode labeling are non-compliant.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration/comparator provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- No heuristic/proxy substitution is allowed when classifying soil-file
  differences; record exact observed deltas.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/artifacts/simimpl25_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/pl14r-comparator-run-provenance-manifest.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-run-provenance-manifest.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001/**`
- `docs/work-packages/README.md`
- `tools/legacy_comparison_suite/**` (evidence-only updates, if required)
- `tests/integration/infile_soil_parser_contract.rs` (only if contract-derived
  comparison tests are required by findings)

## Phase Plan
### Phase A - Intake and Preconditions
- Confirm SIMIMPL25 disposition/handoff authorizes follow-on soil-file
  comparison scope.
- Freeze baseline/candidate lane selection and comparison policy inputs.

### Phase B - Soil-File Evidence Acquisition
- Resolve compared `soil.dat`/`.sol` file paths for baseline and candidate
  lanes.
- Capture reproducible provenance (paths, hashes, file metadata, run context).

### Phase C - Structural and Semantic Delta Classification
- Execute deterministic structural and numeric comparisons.
- Classify differences into format/version, value, and semantic-impact buckets.

### Phase D - Contract/Gate Posture and Governance
- Map findings against canonical soil/input/system authority.
- Run required package gates and complete dual review/verification artifacts.

### Phase E - Disposition
- Record final SIMIMPL26 disposition.
- Keep disposition in `HOLD` when unresolved soil-file deltas are not
  dispositioned with explicit ownership/risk rationale.

## Exit Criteria
- Baseline and candidate soil-file comparison evidence is captured with
  reproducible provenance.
- Delta classification is explicit and traceable to canonical authority.
- Required governance artifacts are complete with truthful `Static:`/`Ran:`
  labeling.
- Required non-doc gates are run and recorded:
  1. `cargo test -p openwepp --test infile_soil_parser_contract`
  2. `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: comparator/provenance assessment package focused on reproducible
  evidence and governance disposition.
