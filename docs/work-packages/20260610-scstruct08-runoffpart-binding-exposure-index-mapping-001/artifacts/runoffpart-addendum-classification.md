# SCSTRUCT08 RUNOFFPART Addendum Classification

Evidence: Static
Date: 2026-06-11
Target: `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`

## Summary

SCSTRUCT08 enumerated 15 top-level `## ... Addendum` sections in
`SC-RUNOFFPART-001` and added one Binding Exposure Index row for each. Two
sections carry same-section `INV-RUNOFFPART-*` references and were mechanically
mapped. Thirteen active sections contain binding language without a same-section
RUNOFFPART binding ID and were routed to SCSTRUCT09.

Classification counts:

| Status | Binding classification | Rows | Gate |
|---|---:|---:|---|
| `active` | `maps-to-existing-INV` | 2 | `none` |
| `active` | `unpromoted-binding` | 13 | `science-review-follow-on` |
| total |  | 15 | 13 deferred |

## Classification Table

| Entry ID | Source lines | Status | Classification | Binding IDs | Gate | Rationale |
|---|---:|---|---|---|---|---|
| `EROD12-CROSS-DOMAIN-OWNERSHIP-AND-GUARD-CLOSURE-ADDENDUM` | 222-228 | `active` | `maps-to-existing-INV` | `INV-RUNOFFPART-007, INV-RUNOFFPART-008, INV-RUNOFFPART-009, INV-RUNOFFPART-011` | `none` | Same-section table names RUNOFFPART producer ownership for runoff/peak-duration and multi-OFE lanes through cited invariants; cross-contract consumer detail stays core-resident. |
| `WB12-RUNOFF-RECONCILIATION-ADDENDUM` | 307-352 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Runoff reconciliation equations, carryover precedence, lane-specific closure deltas, typed guards, and vectors have no same-section RUNOFFPART binding ID. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | 353-371 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | WB13 runoff/runon output and hard-fail requirements have no same-section RUNOFFPART binding ID. |
| `WB14-INFILTRATION-AND-SUBDAILY-HYETOGRAPH-KERNEL-AUTHORITY-ADDENDUM` | 372-483 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Hyetograph, disturbed-conductivity, Green-Ampt, runoff reconciliation, tolerance, guard, and vector obligations have no same-section RUNOFFPART binding ID. |
| `WB15-CANOPY-INTERCEPTION-RUNTIME-COUPLING-ADDENDUM` | 484-536 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Interception ordering, biomass equation-input cap, coupled runoff closure, guards, and vectors have no same-section RUNOFFPART binding ID. |
| `IRRIG10-IRRIGATION-RUNTIME-COUPLING-ADDENDUM` | 537-576 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Irrigation schedule resolution, forcing-depth coupling, runoff equation, guard, and vector obligations have no same-section RUNOFFPART binding ID. |
| `CLIM05-SNOW-RUNTIME-COUPLING-ADDENDUM` | 577-616 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Snow control/state requirements, signed `S` liquid-input coupling, runoff reconciliation, and guard/vector obligations have no same-section RUNOFFPART binding ID. |
| `CLIM06-FROZEN-SOIL-RUNTIME-COUPLING-ADDENDUM` | 617-663 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Frost controls, frozen-state domains, infiltration-capacity consumption, guard, and vector obligations have no same-section RUNOFFPART binding ID. |
| `WB16-PEAK-RUNOFF-KERNEL-ADDENDUM` | 664-762 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Peak-runoff branch equations, baseline near-zero behavior, `m`/`ealpha` producer authority, provenance policy, guards, and vectors have no same-section RUNOFFPART binding ID. |
| `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | 763-783 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Typed symbol/accessor migration obligations and guard-preservation vectors have no same-section RUNOFFPART binding ID. |
| `EROD13-WAVE-1-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | 784-796 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Wave-1 enabled-path mandatory producer surfaces and fallback prohibitions cite RUNOFFPART ownership but no same-section RUNOFFPART binding ID. |
| `EROD14-WAVE-2-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | 797-813 | `active` | `maps-to-existing-INV` | `INV-RUNOFFPART-007, INV-RUNOFFPART-008, INV-RUNOFFPART-009` | `none` | Same-section text preserves continuity from `INV-RUNOFFPART-007..009`; semantic completeness remains for SCSTRUCT09 if relocation is later considered. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-ADDENDUM` | 814-826 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Same-pass carryover precedence, republished carryover, anti-shadow behavior, and malformed flux rejection have no same-section RUNOFFPART binding ID. |
| `HPHYS0241-MOFE-HOURLY-CARRY-ARRAY-RUNOFF-ADDENDUM` | 827-841 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Hourly array-authoritative upstream runon, aggregate anti-shadow, area-scaling provenance, and malformed-array rejection have no same-section RUNOFFPART binding ID. |
| `HPHYS0242-SURFACE-SATURATION-RUNOFF-ADDBACK-ADDENDUM` | 842-855 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | `surdra` addback, same-pass `Q` closure, hidden-storage prohibition, and vector obligations have no same-section RUNOFFPART binding ID. |

## Boundary Confirmation

No addendum narrative was relocated. No invariant, obligation, guard, gap, or
revision-history row was changed. The `Gap Register` is not an addendum section
and was intentionally not indexed by SCSTRUCT08.
