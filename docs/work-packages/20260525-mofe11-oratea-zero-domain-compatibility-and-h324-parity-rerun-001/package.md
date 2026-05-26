# 20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Replicate legacy WEPP `oratea/orater=0` decomposition semantics in openWEPP by
allowing zero-valued decomposition constants (no decay branch) with typed
non-negative guards, then rerun carved-letter `H324` MOFE semantic parity.

## Why This Package Exists
MOFE10 closed `gddmax=0` sentinel compatibility but `H324` parity remains
blocked at management projection:
`HS-RUNTIME-E-050: PL projection field oratea ... out of domain (0, allowed >0.0)`.
Legacy WEPP reads `oratea`/`orater` directly from management input without a
strict positive guard and applies them in `exp(-ENVIND*ORate*)`; zero is
legacy-compatible and produces no decomposition decay.

## Scope
### Included
- Canonical contract amendments for decomposition-rate domain semantics to
  allow zero with explicit no-decay behavior.
- Contract-derived tests for runtime projection and decomposition equation
  behavior when `oratea/orater=0`.
- Pre-implementation gate evidence (expected failing behavior before code edits).
- Runtime implementation to support non-negative decomposition constants while
  preserving typed fail-closed behavior for negative/non-finite values.
- Carved-letter `H324` MOFE rerun and comparator execution when candidate
  outputs are produced.

### Explicitly Out of Scope
- New process-physics equations unrelated to decomposition-rate domain policy.
- Broad management parser redesign beyond scoped decomposition constants.
- Watershed routing redesign.

## Deliverables
1. Legacy `oratea/orater` runtime implementation report:
   - `artifacts/mofe11-oratea-legacy-behavior-implementation-report.md`
2. Legacy `oratea/orater` test matrix:
   - `artifacts/mofe11-oratea-legacy-test-matrix.md`
3. H324 parity rerun report:
   - `artifacts/mofe11-h324-parity-rerun-report.md`
4. Contract implementation evidence:
   - `artifacts/mofe11-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/mofe11-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/mofe11-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/mofe11-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/mofe11-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe11_disposition.md`
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
1. Amend canonical contracts for scoped `oratea/orater` domain authority.
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001/artifacts/mofe10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/infile.for`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for`
- `/workdir/wepp-forest_260430_baseline/src/cdecvar1.inc`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`

## Intended Write Set
- `docs/work-packages/20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `tests/integration/parser_runtime_seam_integration.rs`

## Phase Plan
### Phase A - Contract Authority Alignment
- Amend canonical plant/residue contracts to define non-negative (`>=0`)
  decomposition-rate domain semantics and explicit zero no-decay behavior.
- Record baseline source references (`infile.for`, `decomp.for`, `cdecvar1.inc`).

### Phase B - Contract-Derived Tests
- Add tests proving runtime projection accepts `oratea/orater=0` while rejecting
  negative values.
- Add decomposition equation tests proving zero constants preserve residue/root
  masses under equation decay terms.

### Phase C - Pre-Implementation Contract Gate
- Run targeted tests before production edits and record expected failing posture
  for new zero-domain vectors.

### Phase D - Runtime Implementation
- Implement legacy-compatible decomposition-rate domain handling in
  management projection and decomposition equation input validation.
- Maintain typed error guards; no silent fallback defaults.

### Phase E - MOFE Parity Rerun
- Re-run carved-letter `H324` lane.
- Execute semantic comparator when candidate outputs are produced.
- If blocked, capture next typed blocker with reproducibility evidence.

### Phase F - Closeout
- Complete artifacts, dual review/verification, gate matrix, and disposition.

## Exit Criteria
- `HS-RUNTIME-E-050` no longer blocks valid `oratea/orater=0` payloads.
- Contract-derived tests for zero-domain decomposition constants pass.
- `H324` parity lane rerun executes through comparator or yields a new typed
  blocker after decomposition-rate runtime-surface resolution.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser/runtime coupling + tests/docs only; no credential/network
  boundary changes.
