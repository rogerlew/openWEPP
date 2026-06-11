# SCSTRUCT08 Science Review Follow-On Queue

Evidence: Static
Date: 2026-06-11
Owner package: `20260610-scstruct09-runoffpart-bei-science-review-adjudication-001`
Defect ID: `SCSTRUCT08-RUNOFFPART-BEI-SCIENCE-REVIEW`

## Queue Summary

SCSTRUCT08 routed 13 SC-RUNOFFPART Binding Exposure Index rows to SCSTRUCT09.
The common decision required for each deferred row is:

1. prove the row maps completely to existing `INV-RUNOFFPART-*`/`OBL-RUNOFFPART-*`
   authority,
2. promote precise binding authority through the contract review gate, or
3. retain the row in core with a narrower explicit HOLD.

Until adjudicated, the narrative remains in the `SC-RUNOFFPART-001` core.

## Deferred Rows

| Entry ID | Gate reason | Next evidence gate |
|---|---|---|
| `WB12-RUNOFF-RECONCILIATION-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote runoff reconciliation equations, carryover precedence, lane-specific closure deltas, typed guards, and vectors. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote WB13 `Q`/`QOFE`/`UpStrmQ`/`RM`/`P` output and hard-fail requirements. |
| `WB14-INFILTRATION-AND-SUBDAILY-HYETOGRAPH-KERNEL-AUTHORITY-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote hyetograph, disturbed-conductivity, Green-Ampt, runoff reconciliation, tolerance, guard, and vector authority. |
| `WB15-CANOPY-INTERCEPTION-RUNTIME-COUPLING-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote interception ordering, biomass equation-input cap, coupled runoff closure, guard, and vector authority. |
| `IRRIG10-IRRIGATION-RUNTIME-COUPLING-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote irrigation schedule resolution, forcing-depth coupling, runoff equation, guard, and vector authority. |
| `CLIM05-SNOW-RUNTIME-COUPLING-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote snow control/state requirements, signed `S` liquid-input coupling, runoff reconciliation, and guard/vector authority. |
| `CLIM06-FROZEN-SOIL-RUNTIME-COUPLING-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote frost controls, frozen-state domains, infiltration-capacity consumption, guard, and vector authority. |
| `WB16-PEAK-RUNOFF-KERNEL-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote peak-runoff branch equations, near-zero behavior, `m`/`ealpha` producer authority, provenance policy, guards, and vectors. |
| `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote typed symbol/accessor migration obligations and guard-preservation vectors. |
| `EROD13-WAVE-1-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote Wave-1 producer-surface requirements and fallback prohibitions to exact RUNOFFPART binding authority. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote same-pass carryover precedence, republished carryover, anti-shadow behavior, and malformed flux rejection. |
| `HPHYS0241-MOFE-HOURLY-CARRY-ARRAY-RUNOFF-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote hourly carry-array authority, aggregate anti-shadow, area-scaling provenance, and malformed-array rejection. |
| `HPHYS0242-SURFACE-SATURATION-RUNOFF-ADDBACK-ADDENDUM` | Active binding language without same-section RUNOFFPART binding ID. | Map/promote `surdra` addback, same-pass `Q` closure, hidden-storage prohibition, and vector authority. |
