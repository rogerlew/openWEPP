# 20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Replicate legacy WEPP `gddmax=0` sentinel behavior in openWEPP by resolving
runtime `gddmax` from climate monthly statistics and management day controls,
then rerun carved-letter `H324` MOFE semantic parity.

## Why This Package Exists
MOFE09 closed soil runtime seam blocker `HS-RUNTIME-E-003` but `H324` parity is
blocked by:
`HS-RUNTIME-E-050: PL projection field gddmax ... out of domain (0, allowed >0.0)`.
Legacy WEPP treats `gddmax<=0` as a sentinel that triggers internal
estimation in `yldopt/gdmax` using planting/harvest windows and monthly
climate temperatures; openWEPP currently hard-fails before that legacy
resolution behavior can occur.

## Scope
### Included
- Canonical contract amendments for legacy `gddmax` sentinel resolution
  authority and runtime guard posture.
- Contract-derived tests for annual/perennial `gddmax<=0` resolution paths.
- Pre-implementation gate evidence (expected failing behavior before code edits).
- Runtime implementation for legacy-compatible `gddmax` resolution.
- Watershed climate adapter monthly-vector projection parity required to keep
  climate parser/runtime seam parity green after monthly-symbol introduction.
- Carved-letter `H324` MOFE rerun and comparator execution when candidate
  outputs are produced.

### Explicitly Out of Scope
- New process-physics equations unrelated to legacy `gddmax` resolution.
- Broad management parser redesign beyond scoped sentinel behavior.
- Watershed routing redesign.

## Deliverables
1. Legacy `gddmax` runtime implementation report:
   - `artifacts/mofe10-gddmax-legacy-behavior-implementation-report.md`
2. Legacy `gddmax` test matrix:
   - `artifacts/mofe10-gddmax-legacy-test-matrix.md`
3. H324 parity rerun report:
   - `artifacts/mofe10-h324-parity-rerun-report.md`
4. Contract implementation evidence:
   - `artifacts/mofe10-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/mofe10-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/mofe10-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/mofe10-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/mofe10-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe10_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical requirements or
unresolvable environment failures.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Amend canonical contracts for scoped `gddmax` sentinel authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production runtime code and execute parity rerun.

No production runtime behavior edits are permitted before steps 1-3 complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe09-hs-runtime-e-003-soil-runtime-fallback-and-h324-parity-rerun-001/artifacts/mofe09_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe09-hs-runtime-e-003-soil-runtime-fallback-and-h324-parity-rerun-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/yldopt.for`
- `/workdir/wepp-forest_260430_baseline/src/gdmax.for`
- `/workdir/wepp-forest_260430_baseline/src/grow.for`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`

## Intended Write Set
- `docs/work-packages/20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/parser_runtime_seam_integration.rs`

## Phase Plan
### Phase A - Contract Authority Alignment
- Amend canonical plant contract to define legacy `gddmax<=0` sentinel
  resolution behavior, including annual/perennial branch rules and
  fail-closed posture for unresolved/invalid estimates.
- Record baseline source references (`yldopt.for`, `gdmax.for`, `grow.for`,
  `tilage.for`, and chapter/user-summary anchors).

### Phase B - Contract-Derived Tests
- Add tests proving runtime acceptance and deterministic resolution of
  `gddmax<=0` under annual and perennial management classes when monthly
  climate vectors are available.
- Preserve typed hard-fail behavior for missing monthly vectors,
  out-of-domain day controls, or non-positive resolved `gddmax`.

### Phase C - Pre-Implementation Contract Gate
- Run targeted tests before production edits and record expected failing
  posture for new sentinel-resolution vectors.

### Phase D - Runtime Implementation
- Implement legacy-compatible `gddmax` resolution:
  - project required monthly climate vectors to runtime surfaces;
  - resolve `gddmax` using legacy `gdmax` monthly-temperature integration
    semantics and branch logic from `yldopt`.
- Preserve hillslope/watershed climate-seam symbol parity by adding prefixed
  monthly climate vector projection in watershed assignment adaptation.
- Maintain typed error guards; no silent fallback defaults.

### Phase E - MOFE Parity Rerun
- Re-run carved-letter `H324` lane.
- Execute semantic comparator when candidate outputs are produced.
- If blocked, capture next typed blocker with reproducibility evidence.

### Phase F - Closeout
- Complete artifacts, dual review/verification, gate matrix, and disposition.

## Exit Criteria
- `HS-RUNTIME-E-050` no longer blocks valid `gddmax=0` management payloads
  when legacy-authoritative resolution inputs are available.
- Contract-derived tests for annual/perennial sentinel-resolution paths pass.
- `H324` parity lane rerun executes through comparator or yields a new typed
  blocker after growth runtime surface resolution.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser/runtime coupling + tests/docs only; no credential/network
  boundary changes.
