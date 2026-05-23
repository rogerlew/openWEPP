# PL16 Contract Implementation Evidence

Status: `complete`
Evidence mode: `Static`

## Canonical Contract Amendments Implemented

1. `SC-PLANT-001` updated for PL16 growth-physics authority:
- `contract_version: 8 -> 9`
- Added explicit PL16 algorithm state surfaces and equation-update specification (GDD, stress regulation, biomass, canopy, LAI, root mass/depth, senescence)
- Added PL16 invariants `INV-PLANT-018..021`
- Added legacy growth-equation provenance and constants authority (`REF-PLANT-LEGACY-GROW` and PL16 constants table rows)
- Added PL16 test-vector obligations and closed `GAP-PLANT-008`

2. `SC-RESIDUE-001` updated for PL16 transition semantics:
- `contract_version: 6 -> 7`
- Renamed PL growth branch authority from `PL13` to `PL16`
- Removed senescence-reset from reset-class action set
- Added explicit equation-day branch authority (`BR-RES-PL16-GROWTH-EQUATION`)
- Updated growth reset/test-vector obligations to PL16 behavior

3. `science-contracts/index.md` lifecycle notes updated:
- `SC-PLANT-001` note now records PL16 equation-authoritative runtime behavior
- `SC-RESIDUE-001` note now records PL16 growth-transition alignment and INT10 continuity

## Production Behavior Authority Alignment

Static diff review confirms implementation symbols and guard posture in
`openwepp-hillslope-orchestrator` now match PL16 contract authority for:
- non-reset active growth equation updates,
- reset-class action zero-state payloads,
- hard-fail required-symbol/domain behavior.
