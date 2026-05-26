# 20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Implement SIMIMPL32 contract-derived frost-hourly tests and pre-implementation
contract-gate closure for the SIMIMPL31 routine-authority scope before any
frost runtime/kernel migration edits.

## Why This Package Exists
SIMIMPL31 completed canonical frost routine-authority closure and retained
`HOLD`. Contract-first sequencing now requires test-authoring and a
pre-implementation gate before SIMIMPL33/SIMIMPL34 production frost migration
can begin.

SIMIMPL32 executes those prerequisite steps so downstream implementation
packages can proceed under validated failure/closure expectations.

## Scope
### Included
- Implement contract-derived tests for SIMIMPL31 frost authority surfaces:
  - dispatch-trigger closure (active vs inactive frost coupling),
  - handoff-direction closure (`frwatc(1)` ingress and `frwatc(0)` egress
    semantics represented at runtime seam),
  - freeze-lineage closure (`frzng`/`frznw`-mapped frost-hourly families),
  - conductivity-lineage closure (`frsoil`/`getFreezeCond` mapped behavior),
  - cross-contract frost seam completeness (`SC-SOIL-001`,
    `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SYSTEM-001`).
- Record pre-migration failure baseline evidence showing expected failures on
  current reductive frost behavior where authoritative families are not yet
  migrated.
- Record pre-implementation contract gate evidence required before
  SIMIMPL33/SIMIMPL34 production edits.
- Update governance evidence, review, verification, and downstream handoff
  artifacts for SIMIMPL33/34/35 sequence.

### Explicitly Out of Scope
- Production runtime/kernel code edits (SIMIMPL33/SIMIMPL34 scope).
- Canonical contract-authority rewrites beyond minimal corrective clarifications
  strictly required to keep SIMIMPL32 tests coherent with `SC-SNOWFREEZE-001`.
- Winter-hourly hold-lift rerun/disposition closure (SIMIMPL35 scope).

## Deliverables
1. Contract-derived frost test matrix:
   - `artifacts/simimpl32-contract-derived-test-matrix.md`
2. Pre-migration failure baseline:
   - `artifacts/simimpl32-pre-migration-failure-baseline.md`
3. Contract implementation evidence:
   - `artifacts/simimpl32-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl32-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl32-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl32-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl32-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl32_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages in this wave, sequencing must remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL32 executes steps 2 and 3 for declared frost migration scope.

## Autonomous Execution Intent (Required)
This package is execution-ready and intended for end-to-end autonomous
completion through disposition without additional user direction unless
hard-blocked by contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in acceptance
  claims.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/artifacts/simimpl31_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/frostn.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
- `/workdir/wepp-forest_260430_baseline/src/frzng.for`
- `/workdir/wepp-forest_260430_baseline/src/frznw.for`
- `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`

## Intended Write Set
- `docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/**`
- `docs/work-packages/README.md`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- Minimal contract/index clarifications only if required to resolve
  authoritative contradiction discovered while authoring tests.

## Phase Plan
### Phase A - Intake and Test-Scope Freeze
- Confirm SIMIMPL31 authority outputs and required SIMIMPL32 test vectors.
- Freeze declared write set for tests and governance artifacts.

### Phase B - Contract-Derived Test Authoring
- Implement SIMIMPL31-derived frost test vectors in integration tests.
- Encode typed, explicit failure posture for missing/invalid active frost
  runtime families.

### Phase C - Pre-Implementation Contract Gate
- Execute targeted test commands and record expected-failure baseline where
  current reductive frost implementation is incomplete.
- Record gate evidence for SIMIMPL33/SIMIMPL34 eligibility.

### Phase D - Governance and Handoff
- Complete required evidence, dual reviews, and dual verifications.
- Prepare downstream SIMIMPL33/SIMIMPL34 handoff.

### Phase E - Disposition
- Record final SIMIMPL32 disposition and gate posture.
- Keep disposition in `HOLD` until runtime migration packages close remaining
  frost-hourly implementation gaps.

## Exit Criteria
- Contract-derived tests exist for all SIMIMPL31 frost authority families.
- Expected-failure pre-migration baseline is recorded truthfully.
- Pre-implementation contract gate evidence is recorded and explicit.
- Required governance artifacts are complete with truthful `Static:`/`Ran:` labeling.
- If non-doc files are changed, required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: test-authoring + governance package; no production runtime
  mutation expected.
