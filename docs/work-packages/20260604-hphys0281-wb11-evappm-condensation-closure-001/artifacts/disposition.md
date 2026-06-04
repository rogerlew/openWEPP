# Disposition

Status: completed/HOLD
Evidence mode: static + ran

## Summary

HPHYS0281 implementation is complete. The WB11/EVAPPM condensation defect is corrected, the SIMIMPL18 fixture blocker is green, and full workspace tests pass. The package remains `completed/HOLD` only because the scoped SC-EVAP unit-compliance lint still reports pre-existing HPHYS0279 `Ep`/`Es`/`Er` documentation findings; HPHYS0281 did not add a new unit-lint finding for `pmet.es_storage_return_m`.

## Completed Deliverables

- Added `SC-EVAP-001#INV-EVAP-025` and `pmet.es_storage_return_m` authority.
- Registered `pmet.es_storage_return_m` in executable boundary unit registry.
- Added red/green tests for EVAPPM seed, WB17 storage-return consumption, and WB13 roundoff canonicalization.
- Implemented PMET seed publication of non-negative `pmet.es_m` plus positive storage return.
- Wired WB17 to apply the return during ET phase, preserving phase ordering.
- Removed WB13 dependence on EVAPPM branch-specific negative-`Es` clamp behavior.
- Reran focused SIMIMPL18 and full workspace gates.

## HOLD Reason

`tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md` still reports 11 HPHYS0279-era findings for `Ep`/`Es`/`Er` unit/alias rows. These findings predate HPHYS0281 and are documented as continuation governance debt.

## Review Finding Disposition

- Review A HIGH negative active-canopy `ep`: accepted and fixed by
  canonicalizing negative raw `ep` to zero published `pmet.ep_m` and zero
  `wb11_et_demand`; focused and workspace gates pass after the change.
- Review A MEDIUM untyped `pmet.es_storage_return_m`: accepted and fixed by
  publishing the storage return as `BoundaryValue::water_depth_meters`.
- Review B BLOCKER clippy `similar_names`: accepted and fixed by renaming local
  variables; workspace clippy passes after the change.
- Review B MEDIUM zero-residue producer fixture: accepted and fixed by using
  nonzero residue in the producer test while retaining WB17 combined
  residue-plus-storage-return closure assertions.
- Review A/B artifact blockers: accepted; review artifacts are populated.

## Verification Finding Disposition

- Verification A BLOCKER queued dual verification artifacts: accepted and
  resolved by populating both verification artifacts.
- Verification B BLOCKER queued dual verification artifacts: accepted and
  resolved by populating both verification artifacts.
- Verification B MEDIUM producer fixture mismatch: accepted and fixed by
  setting the HPHYS0281 producer fixture `wb17_residue_interception` to
  `0.000_2`; focused HPHYS0281 tests, workspace clippy, docs lint, diff
  hygiene, and full workspace tests pass after the fix.
- Verification B MEDIUM incomplete HOLD reason: accepted; after verification
  artifacts and fixture correction, the only remaining HOLD reason is the
  pre-existing SC-EVAP unit-compliance debt documented above.
