# PL02 Follow-On Implementation Handoff

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL01 queue defines `PL03` as the next executable dependency after PL02.

Ran:
- Consolidated PL02 boundary outputs into an implementation-ready checklist for PL03/PL04 owners.

## Immediate Next Package Targets

1. `PL03` Management-to-Runtime Adapter
- Implement `PL-MAN-SEAM-001` typed adapter in hillslope runtime input boundary.
- Output typed `pl_schedule`, `pl_growth`, and `pl_decomp` surfaces.
- Enforce strict typed errors for missing/invalid required fields.

2. `PL04` PL Symbol Alias Completion
- Extend `openwepp-sim-contract` canonical registry with PL symbol coverage and template aliases.
- Add tests for forward and reverse alias resolution on new PL symbols.

## Required Acceptance Conditions for PL03

1. No silent defaults for required PL controls/state.
2. Typed error taxonomy is explicit and tested.
3. Adapter preserves branch semantics (`landuse`, `imngmt`, `resmgt`, `mgtopt`) from parser output.
4. Scheduler-facing surfaces preserve decomposition-before-soil/watbal ordering prerequisites.
5. Integration evidence proves parser-to-runtime PL projection from `.man` fixtures.

## Required Acceptance Conditions for PL04

1. PL canonical symbols are present in registry with deterministic alias patterns.
2. Alias templates remain compatible with existing token validation policy.
3. No ambiguous alias back-mapping introduced.

## Carry-Forward Risks

1. Rangeland (`landuse=2`) execution remains out of current runtime scope and must stay explicit in status/disposition language.
2. Growth and decomposition kernels are not implemented in PL02; PL02 only closes contract/ownership surfaces.
3. Comparator confidence-tier PL review (`PL08`) remains blocked until PL05/PL06/PL07 close.

## Evidence Links

- `/home/workdir/openWEPP/docs/work-packages/20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001/artifacts/plant-landuse-growth-decomposition-follow-on-wp-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-boundary-contract.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-seam-requirements.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-canonical-symbol-alias-requirements.md`
