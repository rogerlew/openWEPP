# HPHYS0240 Disposition

Status: completed
Evidence mode: Static + Ran

Decision: COMPLETE_GROUP_B_CARRYOVER_CLOSED_HPHYS_HOLD_FOR_GROUPS_C_D

Static: closure summary:

- HPHYS0240 closes the HPHYS0239 Dispatch-Group-B residual for WB14/WB12
  same-pass runoff carryover.
- Canonical contracts now define `wb12_runoff_carryover` authority and guard
  posture across `SC-WATBAL-001`, `SC-RUNOFFPART-001`, and `SC-SUBHYD-001`.
- Production runoff reconciliation now prefers same-pass carryover flux, falls
  back to `wb12_runon_input` only when carryover flux is absent, validates
  malformed present fluxes, and republishes resolved carryover.
- Scheduler dependencies were already correct for this carryover path:
  `Drainage -> RunoffReconciliation -> StorageReconciliation`.

Ran: validation passed:

- Focused HPHYS0240 tests passed after implementation.
- Full modified integration test files passed.
- Required workspace gates passed, including `cargo test --workspace`.

Residual posture:

- Dispatch Group B is closed for this package scope.
- HPHYS stream remains in `HOLD` for already-scaffolded follow-up packages:
  - `20260601-hphys0241-mofe-hourly-carry-arrays-routing-continuity-001`
  - `20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001`
