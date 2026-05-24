# 20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Produce a thorough, implementation-driving assessment of hillslope legacy
routines not yet implemented in openWEPP, with explicit coverage of the active
`cli -> runner -> simulation -> orchestration` pipeline and a concrete
implementation queue for closing identified gaps.

## Why This Package Exists
Current runner-generated WAT candidate behavior demonstrates that parts of the
runtime path remain projection/synthesis-driven rather than full legacy-
equivalent hillslope simulation closure. The repository also lacks a complete
openWEPP equivalent of legacy `watbal.for` / `watbal_hourly.for` behavior.

This package creates authoritative assessment outputs needed to start a
contract-first implementation wave that:
1. identifies missing legacy hillslope routines and branch behavior,
2. maps those gaps to openWEPP pipeline surfaces,
3. decides legacy source authority for watbal implementation basis
   (`/workdir/wepp-forest_260430_baseline` vs `/workdir/wepp-forest`), and
4. emits an executable work-package queue for implementation.

## Scope
### Included
- Inventory hillslope legacy routines under
  `/workdir/wepp-forest_260430_baseline/` and classify implementation status in
  openWEPP.
- Perform top-down review of the openWEPP execution path:
  - CLI inputs/flags,
  - runner runtime-surface projection,
  - simulation kernel invocation and state publication,
  - orchestration phase ordering and output emission.
- Confirm and document absence/incompleteness of full watbal-equivalent runtime
  behavior in openWEPP.
- Assess legacy authority candidates for watbal implementation basis:
  - snapshot baseline: `/workdir/wepp-forest_260430_baseline`
  - consolidated candidate: `/workdir/wepp-forest`
- Produce watbal/watbal_hourly consolidation design requirements for openWEPP:
  - `wepp_ui`-controlled hourly branch,
  - consolidated single kernel implementation,
  - extensible timestep framework with future support for sub-hourly cadence
    (e.g., `0.25 h`, `6/60 h`).
- Produce implementation queue deliverable:
  - `artifacts/simulation-implementation-wp-queue.md`.

### Explicitly Out of Scope
- Direct production implementation of watbal/hydrology kernels.
- Closure disposition of erosion/sediment parity.
- Watershed production-kernel authoring beyond assessment dependencies.

## Deliverables
1. Contract/authority assessment evidence:
   - `artifacts/simimpl01-contract-implementation-evidence.md`
2. Legacy hillslope routine gap register:
   - `artifacts/simimpl01-hillslope-routine-gap-register.md`
3. Pipeline audit (`cli -> runner -> simulation -> orchestration`):
   - `artifacts/simimpl01-pipeline-gap-audit.md`
4. Legacy source authority comparison (`260430` vs `wepp-forest`):
   - `artifacts/simimpl01-watbal-authority-source-comparison.md`
5. Watbal/watbal_hourly consolidation requirements and extensibility note:
   - `artifacts/simimpl01-watbal-consolidation-and-timestep-architecture.md`
6. Contract-test assessment evidence:
   - `artifacts/simimpl01-contract-test-implementation-evidence.md`
7. Pre-implementation contract gate:
   - `artifacts/simimpl01-preimplementation-contract-gate.md`
8. Implementation/test evidence artifact (assessment-run commands + outputs):
   - `artifacts/simimpl01-implementation-and-test-evidence.md`
9. Kernel-profile compliance checklist:
   - `artifacts/simimpl01-kernel-profile-compliance-checklist.md`
10. Implementation queue deliverable (required):
   - `artifacts/simulation-implementation-wp-queue.md`
11. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl01_disposition.md`
   - `artifacts/worker-handoff.md`
12. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages spawned from this assessment, sequence
must remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

No implementation queue item may be marked executable if it violates this order.

## Autonomous Execution Intent (Required)
This package must be execution-ready and self-contained. Assigned agents are
expected to progress phase-by-phase through disposition and artifact updates
without requesting additional user direction unless hard-blocked by missing
local authority, unreadable inputs, or contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Physics and Authority Posture
- Default legacy provenance anchor is
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- This package must explicitly evaluate whether `/workdir/wepp-forest`
  consolidated watbal implementation (single f90 kernel replacing duplicated
  `watbal`/`watbal_hourly`) is the better implementation basis for openWEPP
  than the pinned 260430 snapshot.
- No physics invention is permitted; each proposed implementation queue item
  must cite canonical contract authority and legacy/source provenance.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001/artifacts/cli04_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/artifacts/pl14s-tier-a-semantic-parity-delta-report.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/**`
- `docs/work-packages/README.md`
- (assessment-dependent, if authority clarifications are required)
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Authority Alignment
- Confirm package authority, dependency readability, and assessment guard
  posture.
- Record initial assumptions for watbal/watbal_hourly legacy source comparison.

### Phase B - Legacy/OpenWEPP Gap Inventory
- Build complete hillslope routine inventory from baseline legacy source.
- Map openWEPP implementation status by routine and pipeline stage.

### Phase C - Pipeline and Branching Assessment
- Audit runtime behavior across `cli -> runner -> simulation -> orchestration`.
- Identify branch-gating requirements for `wepp_ui` hourly path and document
  gaps vs current behavior.

### Phase D - Consolidation Design and Queue Authoring
- Produce consolidation requirements for unified watbal kernel with
  extensible timestep cadence.
- Produce `simulation-implementation-wp-queue.md` with dependency-aware,
  contract-first implementation packages.

### Phase E - Verification and Disposition
- Complete dual review/verification artifacts and gate/disposition updates.
- Keep disposition in `HOLD` if unresolved authority or closure blockers remain.

## Exit Criteria
- Complete hillslope routine implementation-gap register exists with explicit
  per-routine status and evidence links.
- Pipeline audit identifies concrete missing behaviors for each stage in
  `cli -> runner -> simulation -> orchestration`.
- Watbal source-authority comparison includes a clear decision recommendation
  (`260430` vs consolidated `/workdir/wepp-forest`) with rationale.
- Consolidation design requirements explicitly cover:
  - shared watbal kernel,
  - `wepp_ui` hourly-branch control,
  - future sub-hourly extensibility.
- `artifacts/simulation-implementation-wp-queue.md` exists and enumerates
  executable follow-on packages with contract-first sequencing.
- Required repository gates are run and recorded if code changes beyond docs
  are introduced:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: assessment/planning package; no production kernel execution
  pathways are modified by this scaffold.

## Execution Record
- 2026-05-24: Phase A kickoff prompt executed; authority baseline established
  across canonical contracts, baseline provenance, and consolidated candidate
  sources.
- 2026-05-24: Phase B completed; full legacy routine inventory extracted for
  assessed hillslope stack and mapped to openWEPP owner surfaces.
- 2026-05-24: Phase C completed; production pipeline audit confirmed
  projection-first output emission and missing runner-to-scheduler execution
  closure.
- 2026-05-24: Phase D completed; consolidation architecture requirements and
  dependency-ordered implementation queue authored.
- 2026-05-24: Phase E completed; gate, review, verification, disposition, and
  worker-handoff artifacts authored and package closed.
