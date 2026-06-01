# 20260601-soilauth01-soil-producer-contract-conformance-audit-001

## Status
- state: queued
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute SOILAUTH01 to produce an authoritative `.sol` producer-contract
conformance audit across `7778/9002/9003/9005`, comparing:
1. openWEPP producer contract text (`soil-file.spec.md`),
2. openWEPP parser/runtime contract (`SC-INFILE-SOIL-001` + parser surfaces),
3. canonical producer behavior in `wepppy`.

## Why This Package Exists
Soil producer authority was re-anchored to `wepppy`, but we still need a
field-level conformance ledger before changing producer/parser code. SOILAUTH01
establishes the deterministic mismatch inventory and closure queue for
SOILAUTH02.

## Scope
### Included
- Build a datver-by-datver matrix for OFE header shape, policy rows,
  per-layer fields/order/units, restrictive-layer semantics, and compatibility
  forms.
- Classify every divergence as one of:
  - contract defect,
  - parser defect,
  - producer defect,
  - intentional compatibility behavior (with provenance).
- Publish a prioritized SOILAUTH02 implementation queue with explicit ownership
  and closure conditions.

### Explicitly Out of Scope
- Any production parser/runtime/proc-physics edits.
- Any anti-drift automation/guard implementation (SOILAUTH03 scope).
- Non-soil input-file surfaces.

## Closure Measures (Required)
1. `MEASURE-SA01-001`: conformance matrix covers all four datver families
   (`7778/9002/9003/9005`) and all declared producer fields.
2. `MEASURE-SA01-002`: every detected mismatch is dispositioned with
   provenance and severity (`P0/P1/P2`).
3. `MEASURE-SA01-003`: SOILAUTH02 fix queue is fully enumerated with explicit
   file targets and test/fixture obligations.

## Deliverables
1. `artifacts/soilauth01-producer-conformance-matrix.md`
2. `artifacts/soilauth01-contract-implementation-evidence.md`
3. `artifacts/soilauth01-contract-test-implementation-evidence.md`
4. `artifacts/soilauth01-preimplementation-contract-gate.md`
5. `artifacts/soilauth01-implementation-and-test-evidence.md`
6. `artifacts/soilauth01-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/soilauth01_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Audit and classify producer/contract/parser conformance for `.sol`.
2. Draft required contract + test deltas for mismatch closure.
3. Record pre-implementation gate evidence that lists unresolved mismatch set.
4. Publish execution-ready SOILAUTH02 fix queue (no production edits in this
   package).

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical producer-contract authority for `.sol` is
  `docs/specifications/wepp-input-files/specs/soil-file.spec.md`.
- Canonical parser/runtime acceptance authority is
  `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`.
- Canonical producer implementation reference is `wepppy`:
  `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py`,
  `/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py`,
  `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/README.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `/workdir/wepppy/wepppy/weppcloud/routes/usersum/input-file-specifications/soil-file.spec.md`
- `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py`
- `/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py`
- `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-soilauth01-soil-producer-contract-conformance-audit-001/**`
- `docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/package.md` (queue inputs only)

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm SOILAUTH01 authorization and freeze scope to `.sol` producer
  conformance only.

### Phase B - Contract/spec authority updates
- Build canonical source inventory and enumerate symbols/arity/order by datver.

### Phase C - Contract-derived tests
- Produce full producer-contract/parser conformance matrix.

### Phase D - Pre-implementation contract gate
- Classify mismatches, severity, and closure target (`contract` vs `parser`
  vs `producer`).

### Phase E - Production implementation
- Publish SOILAUTH02 execution-ready closure queue and update gate artifacts.

### Phase F - Validation and parity rerun
- Run documentation/package validation commands and record evidence labels.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and disposition for audit
  completion.

## Exit Criteria
- Closure measures `MEASURE-SA01-001..003` are satisfied.
- SOILAUTH02 has a deterministic mismatch closure queue with no ambiguous
  ownership.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation/audit artifacts only; no credential/network changes.
