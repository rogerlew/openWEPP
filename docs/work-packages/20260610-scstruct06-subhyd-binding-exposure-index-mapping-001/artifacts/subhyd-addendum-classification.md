# SCSTRUCT06 SUBHYD Addendum Classification

Evidence: Static
Date: 2026-06-11
Target: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`

## Summary

SCSTRUCT06 enumerated 22 top-level `## ... Addendum` sections in
`SC-SUBHYD-001` and added one Binding Exposure Index row for each. Seven
sections carry same-section `INV-SUBHYD-*` references and were mechanically
mapped. Fifteen active sections contain binding language without a same-section
SUBHYD binding ID and were routed to SCSTRUCT07.

Classification counts:

| Status | Binding classification | Rows | Gate |
|---|---:|---:|---|
| `active` | `maps-to-existing-INV` | 7 | `none` |
| `active` | `unpromoted-binding` | 15 | `science-review-follow-on` |
| total |  | 22 | 15 deferred |

## Classification Table

| Entry ID | Source lines | Status | Classification | Binding IDs | Gate | Rationale |
|---|---:|---|---|---|---|---|
| `WB12-RECONCILIATION-COUPLING-ADDENDUM` | 409-434 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | `Qd` reconciliation, carryover precedence, guard, and vector obligations have no same-section SUBHYD binding ID. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | 435-458 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | WB13 subsurface/drainage publication, flux authority, ordering, and guard obligations have no same-section SUBHYD binding ID. |
| `HPHYS0203-SUBSURFACE-WB13-ROBUSTNESS-VALIDATION-ADDENDUM` | 459-470 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Robustness vectors and WB13 `latqcc`/`Dp` guard obligations have no same-section SUBHYD binding ID. |
| `HPHYS0234-WB13-SUBSURFACE-FLUX-AUTHORITY-ANTI-SHADOW-ADDENDUM` | 471-484 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | WB13 flux-over-state anti-shadow authority has no same-section SUBHYD binding ID. |
| `HPHYS0208-COUPLED-SUBSURFACE-RESIDUAL-CLOSURE-ADDENDUM` | 485-496 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Coupled WB11/WB18 seed lineage and fail-closed closure obligations have no same-section SUBHYD binding ID. |
| `HPHYS0218-WB19-DRFC-THRESHOLD-LINEAGE-ADDENDUM` | 497-506 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | WB19 `drfc`/`coca` threshold-lineage obligations have no same-section SUBHYD binding ID. |
| `HPHYS0221-WB19-WATER-YIELD-AND-SATURATED-DEPTH-COUPLING-ADDENDUM` | 507-526 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-024` | `none` | Same-section text references HPHYS0247 baseline `meblfc` authority from `INV-SUBHYD-024`; semantic completeness remains for SCSTRUCT07 if relocation is later considered. |
| `HPHYS0222-WB19-SOLWPV-BRANCH-AUTHORITY-CORRECTION-ADDENDUM` | 527-540 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-015, INV-SUBHYD-024` | `none` | Same-section text links `solwpv` mutation authority to `INV-SUBHYD-015` and saturated-layer selection to `INV-SUBHYD-024`. |
| `HPHYS0224-WB19-REALIZED-WITHDRAWAL-SOIL-WATER-CAP-ADDENDUM` | 541-556 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-016` | `none` | Same-section Level-4 suite linkage names `INV-SUBHYD-016`. |
| `HPHYS0225-WB19-LAYER-POOL-AVAILABLE-CAP-AUTHORITY-ADDENDUM` | 557-570 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-017` | `none` | Same-section Level-4 suite linkage names `INV-SUBHYD-017`. |
| `HPHYS0226-WB19-LATERAL-SATURATED-THICKNESS-RESPONSE-ADDENDUM` | 571-581 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-018` | `none` | Same-section Level-4 suite linkage names `INV-SUBHYD-018`. |
| `HPHYS0227-WB19-FCWP-COCA-WATER-YIELD-COUPLING-ADDENDUM` | 582-598 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-019` | `none` | Same-section Level-4 suite linkage names `INV-SUBHYD-019`. |
| `HPHYS0238-WB19-HOURLY-ITERATIVE-LATERALDRAINAGE-ADDENDUM` | 599-616 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Hourly iterative lane, cumulative cap, and divergence-vector obligations have no same-section SUBHYD binding ID. |
| `HPHYS0239-WB19-WB12WB13-HANDOFF-ORDERING-ADDENDUM` | 617-633 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | WB19 same-pass handoff, flux-authority, and stale-state vector obligations have no same-section SUBHYD binding ID. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-HANDOFF-ADDENDUM` | 634-645 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Same-pass carryover precedence and malformed-carryover rejection obligations have no same-section SUBHYD binding ID. |
| `HPHYS0242-HOURLY-DRAINAGELATERALSATURATION-TAIL-ADDENDUM` | 646-661 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Baseline hourly tail order, final `Qd`, MOFE carry, and vector obligations have no same-section SUBHYD binding ID. |
| `HPHYS0247-WB19-BASELINE-SATURATED-ZONE-CAPACITY-ADDENDUM` | 662-690 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Hourly saturated-zone capacity, `meblfc`, `fffx`, and legacy multiplier obligations have no same-section SUBHYD binding ID. |
| `HPHYS0252-WB19-FROZEN-ADJUSTED-LATERAL-STORAGE-ADDENDUM` | 691-713 | `active` | `maps-to-existing-INV` | `INV-SUBHYD-024` | `none` | Same-section text preserves unfrozen `drfc` conductivity weighting under `INV-SUBHYD-024`; frozen-storage residue remains for semantic review if relocation is considered. |
| `HPHYS0256-WB19-DAILY-LATERAL-LANE-BRANCH-ADDENDUM` | 714-733 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Daily/hourly lane branch, conductivity, and test-vector obligations have no same-section SUBHYD binding ID. |
| `HPHYS0257-WB19-HOURLY-HORIZONTAL-CONDUCTIVITY-ADDENDUM` | 734-755 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Hourly `ui_ssh`/`wb19_lateral_ssh_####` lineage and fail-closed obligations have no same-section SUBHYD binding ID. |
| `HPHYS0258-WB19-HOURLY-CAPWITHDRAWAL-PUBLICATION-ADDENDUM` | 756-771 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Potential/target/realized lateral publication diagnostics and cap-lineage vector obligations have no same-section SUBHYD binding ID. |
| `HPHYS0259-WB19-TRACE-LOCALIZATION-ADDENDUM` | 772-790 | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | Trace serialization and same-surface identity obligations have no same-section SUBHYD binding ID. |

## Boundary Confirmation

No addendum narrative was relocated. No invariant, obligation, guard, gap, or
revision-history row was changed. The `Gap Register` is not an addendum section
and was intentionally not indexed by SCSTRUCT06.
