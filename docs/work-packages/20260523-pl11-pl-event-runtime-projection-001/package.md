# 20260523-pl11-pl-event-runtime-projection-001

## Status
- state: queued
- date: 2026-05-23
- timezone: UTC

## Objective
Expand PL runtime projection to include annual extension controls and perennial
cut/grazing event-day and cycle payload arrays with deterministic symbol naming,
typed bounds checks, and contract-first algorithm authority.

## Why This Package Exists
PL09 identified `PL09-GAP-004` and `PL09-GAP-005` as blocking gaps: event-day
arrays/cycle payloads and annual extension payloads are parsed but not projected
into runtime surfaces. PL10 cleared active slot/crop authority and dispositioned
`GO_FOR_PL11`.

PL10b inserts a contract-first blind-authority and conformance gate. PL11
execution depends on PL10b completion so projection implementation follows
ratified contract intent and reconciled gap outcomes.

This package also codifies kernel-governance posture requested by operator
direction: process intent and algorithm authority must be explicit in
science-contract artifacts before implementation is considered complete.

## Scope
### Included
- Project perennial event-day and cycle payload arrays into runtime symbol
  surfaces with deterministic indexed naming.
- Project annual extension payload controls into runtime symbol surfaces.
- Add/extend typed projection errors for invalid cardinality, bounds,
  non-finite values, and unsupported payload combinations.
- Author contract-first algorithm authority for projected transition-control
  surfaces (top-down provenance, canonical symbols, guard expectations).
- Add fixture-backed projection tests covering annual extension and perennial
  cycles.
- Close PL10b conformance-gate failures captured as ignored tests in
  `tests/integration/parser_runtime_seam_integration.rs`:
  `pl10b_contract_conformance_requires_annual_extension_projection_symbols`,
  `pl10b_contract_conformance_requires_perennial_cutday_indexed_projection`,
  `pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection`,
  `pl10b_contract_conformance_rejects_invalid_grazing_window_domain`, and
  `pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality`.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Production decomposition/residue kinetics execution (`PL12`).
- Production growth transition kinetics execution (`PL13`).
- Tier-A comparator closeout execution (`PL14/PL15`).

## Deliverables
1. Process-contract authority statement:
   - `artifacts/pl11-process-contract-authority.md`
2. Event runtime projection algorithm contract note:
   - `artifacts/pl11-event-runtime-projection-contract.md`
3. Canonical science-contract update plan/evidence:
   - `artifacts/pl11-sc-plant-001-amendment-plan.md`
4. Symbol projection map for annual/perennial payload families:
   - `artifacts/pl11-symbol-projection-map.md`
5. Implementation and test evidence:
   - `artifacts/pl11-implementation-and-test-evidence.md`
6. Typed-seam non-regression evidence:
   - `artifacts/pl11-typed-seam-non-regression-evidence.md`
7. Kernel profile compliance checklist:
   - `artifacts/pl11-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl11_disposition.md`
9. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10-active-slot-authority-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10-active-slot-authority-001/artifacts/pl10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10b-contract-blind-authority-and-conformance-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10b-contract-blind-authority-and-conformance-001/artifacts/pl10b_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs`
- `/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-sim-contract/src/symbols.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-pl11-pl-event-runtime-projection-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL11 scope against queue objective and PL09 gap IDs.

### Phase 1 - Contract Authority
- Ratify projection algorithm authority and canonical symbol obligations in
  `SC-PLANT-001` before considering implementation complete.

### Phase 2 - Implementation
- Extend runtime projection surfaces for annual and perennial payload families
  with deterministic naming and typed guards.

### Phase 3 - Verification
- Add fixture-backed projection tests and run required gates.

### Phase 4 - Disposition
- Publish contract amendment evidence, implementation evidence, and
  review/verification artifacts.

## Exit Criteria
- `PL09-GAP-004` and `PL09-GAP-005` are closed for runtime projection scope.
- Runtime projection emits deterministic indexed symbols for annual extension
  and perennial event/cycle payload families.
- Projection failures are typed and non-silent for bounds/cardinality/domain
  violations.
- `SC-PLANT-001` includes contract-level authority for projected
  transition-control payload semantics and guards.
- `SC-PLANT-001` changes satisfy the kernel profile requirements in
  `kernel-process-contract-profile.md`, with checklist evidence captured.
- PL10b ignored contract-conformance tests listed in Scope are executed
  explicitly and pass.
- PL10b disposition is complete and any reconciled contract-vs-implementation
  gaps required for PL11 start are closed or explicitly accepted.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: projection-surface/contract/test changes only.
