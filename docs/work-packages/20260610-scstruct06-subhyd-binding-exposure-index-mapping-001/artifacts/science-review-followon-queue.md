# SCSTRUCT06 Science Review Follow-On Queue

Evidence: Static
Date: 2026-06-11
Owner package: `20260610-scstruct07-subhyd-bei-science-review-adjudication-001`
Defect ID: `SCSTRUCT06-SUBHYD-BEI-SCIENCE-REVIEW`

## Queue Summary

SCSTRUCT06 routed 15 SC-SUBHYD Binding Exposure Index rows to SCSTRUCT07. The
common decision required for each deferred row is:

1. prove the row maps completely to existing `INV-SUBHYD-*`/`OBL-SUBHYD-*`
   authority,
2. promote precise binding authority through the contract review gate, or
3. retain the row in core with a narrower explicit HOLD.

Until adjudicated, the narrative remains in the `SC-SUBHYD-001` core.

## Deferred Rows

| Entry ID | Gate reason | Next evidence gate |
|---|---|---|
| `WB12-RECONCILIATION-COUPLING-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote `Qd` storage-reconciliation consumption, carryover precedence, guard, and vector authority. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote WB13 subsurface/drainage publication, flux authority, ordering, and guard authority. |
| `HPHYS0203-SUBSURFACE-WB13-ROBUSTNESS-VALIDATION-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote robustness vector and `latqcc`/`Dp` guard authority. |
| `HPHYS0234-WB13-SUBSURFACE-FLUX-AUTHORITY-ANTI-SHADOW-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote WB13 flux-authority anti-shadow authority. |
| `HPHYS0208-COUPLED-SUBSURFACE-RESIDUAL-CLOSURE-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote coupled WB11/WB18 seed-lineage and fail-closed residual closure authority. |
| `HPHYS0218-WB19-DRFC-THRESHOLD-LINEAGE-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote WB19 `drfc`/`coca` threshold-lineage authority. |
| `HPHYS0238-WB19-HOURLY-ITERATIVE-LATERALDRAINAGE-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote hourly iterative lane, cumulative cap, and divergence-vector authority. |
| `HPHYS0239-WB19-WB12WB13-HANDOFF-ORDERING-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote same-pass WB19 handoff, flux-authority, and stale-state vector authority. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-HANDOFF-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote carryover precedence and malformed-carryover rejection authority. |
| `HPHYS0242-HOURLY-DRAINAGELATERALSATURATION-TAIL-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote hourly tail ordering, final `Qd`, MOFE carry, and vector authority. |
| `HPHYS0247-WB19-BASELINE-SATURATED-ZONE-CAPACITY-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote hourly saturated-zone capacity, `meblfc`, `fffx`, and legacy multiplier authority. |
| `HPHYS0256-WB19-DAILY-LATERAL-LANE-BRANCH-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote daily/hourly lane branch, conductivity, and test-vector authority. |
| `HPHYS0257-WB19-HOURLY-HORIZONTAL-CONDUCTIVITY-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote hourly horizontal-conductivity lineage and fail-closed authority. |
| `HPHYS0258-WB19-HOURLY-CAPWITHDRAWAL-PUBLICATION-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote potential/target/realized lateral publication diagnostics and cap-lineage vector authority. |
| `HPHYS0259-WB19-TRACE-LOCALIZATION-ADDENDUM` | Active binding language without same-section SUBHYD binding ID. | Map/promote trace serialization and same-surface identity authority. |
