# 20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL09 end-to-end by implementing the hourly lane foundation with a
typed timestep policy surface (`daily`, `hourly`, future sub-hourly
representable) and adapter-boundary closure, using only the SIMIMPL08 bounded
`adopt` intake set.

## Why This Package Exists
SIMIMPL07 closed mode propagation and typed lane-selection guards. SIMIMPL08
closed consolidated-kernel intake triage and bounded the authorized intake set.
SIMIMPL09 now performs the production implementation wave that makes hourly lane
execution run through explicit typed timestep policy surfaces without importing
rejected/deferred policy overlays.

## Scope
### Included
- Implement typed timestep policy surface(s) for runtime execution lane control
  (`daily`, `hourly`; future sub-hourly representable but not physics-enabled).
- Integrate SIMIMPL08 `adopt` surfaces only:
  - `watbal_process_types` structural mapping,
  - bounded shared kernel family pattern (`wbk01..wbk08` + closure diagnostics),
  - daily/hourly adapter structural boundaries,
  - closure residual guard translated to typed error semantics.
- Close adapter-boundary wiring for hourly execution path using propagated
  `wepp_ui` requested/effective mode from SIMIMPL07.
- Add/activate tests needed to prove typed policy surface behavior and boundary
  closure for implemented scope.
- Complete governance/review/verification/disposition artifacts.

### Explicitly Out of Scope
- Intake of SIMIMPL08 `reject` surfaces (`qcap` overlay, env toggles,
  probe/trace controls, legacy shim/defer wrappers).
- Intake of SIMIMPL08 `defer` surfaces (`wbk19a_*`, route/imp kernels,
  legacy binary pass adapter).
- Winter/frozen-soil/coupling closure wave (`SIMIMPL10`).
- Tier-A replay recloseout and residual classification (`SIMIMPL11`).
- Enabling sub-hourly physics execution (representation scaffolding only).

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl09-contract-implementation-evidence.md`
2. Contract-test prerequisite evidence:
   - `artifacts/simimpl09-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl09-preimplementation-contract-gate.md`
4. Implementation/test evidence log:
   - `artifacts/simimpl09-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl09-kernel-profile-compliance-checklist.md`
6. Hourly lane integration map:
   - `artifacts/simimpl09-hourly-lane-integration-map.md`
7. Timestep policy surface map:
   - `artifacts/simimpl09-timestep-policy-surface-map.md`
8. Adapter-boundary closure matrix:
   - `artifacts/simimpl09-adapter-boundary-closure-matrix.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl09_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL09 must execute in this order:
1. confirm canonical contract authority for SIMMODE/SIMCONS remains complete and
   authoritative (SIMIMPL03),
2. confirm contract-derived tests/pre-gate prerequisites are present and valid
   (SIMIMPL04) and upstream production closures remain intact (SIMIMPL05-08),
3. record SIMIMPL09 pre-implementation contract gate evidence,
4. then implement production hourly-lane/timestep-policy edits.

No production code edits before step 3 is complete.

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents must execute
all phases through disposition without requesting additional user direction
unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Baseline comparator/provenance authority remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Consolidated intake authority for this package is the SIMIMPL08 bounded
  allow-list and exclusions:
  - `simimpl08-adoption-boundary-recommendation.md`
  - `simimpl08-provenance-triage-matrix.md`
- No physics invention is permitted.
- No silent fallback/clamping wrappers are permitted for promoted production
  paths.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-expected-fail-pass-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl06-simulation-owned-wb13-output-publication-001/artifacts/simimpl06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001/artifacts/simimpl08-adoption-boundary-recommendation.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001/artifacts/simimpl08-provenance-triage-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001/artifacts/simimpl08_disposition.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/**`
- `crates/openwepp-sim-contract/src/**`
- `crates/openwepp-kernel-contract/src/**`

## Phase Plan
### Phase A - Contract/Test/Intake Prerequisite Intake
- Confirm SIMIMPL03 authority, SIMIMPL04 test/gate prerequisites, SIMIMPL07
  mode-closure outputs, and SIMIMPL08 bounded intake constraints.

### Phase B - SIMIMPL09 Pre-Implementation Contract Gate
- Record SIMIMPL09 pre-implementation gate evidence and release conditions for
  production edits.

### Phase C - Typed Timestep Policy Surface Implementation
- Implement typed timestep policy surface plumbing for daily/hourly lane
  execution with future sub-hourly representability.

### Phase D - Hourly Adapter-Boundary Integration
- Wire hourly lane adapter boundary execution through admitted surfaces only.
- Enforce explicit typed guards for unsupported/deferred/rejected pathways.

### Phase E - Validation and Closure Evidence
- Run targeted tests and full required gates for touched code.
- Record pass/fail evidence and update integration/policy matrices.

### Phase F - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Hourly lane executes through explicit typed timestep policy surface.
- Sub-hourly representation exists only as scaffold; no physics activation.
- SIMIMPL08 rejected/deferred surfaces are not integrated into production path.
- Typed guard behavior is present for unsupported/deferred paths.
- Required tests/gates for touched code are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Governance/review/verification artifacts are complete with no queued
  placeholders.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal execution-lane/timestep integration; no network or
  privilege-surface expansion.
