# SCSTRUCT07 SUBHYD Row Adjudication Ledger

Evidence: Static
Date: 2026-06-11
Target: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`

## Summary

SCSTRUCT07 adjudicated the 15 rows routed from SCSTRUCT06 and refined two
partially mapped rows. All 22 `SC-SUBHYD-001` Binding Exposure Index rows now
map to existing `INV-SUBHYD-*` authority. No row required promotion, historical
relocation, or a narrower HOLD.

| Outcome | Count |
|---|---:|
| `maps-to-existing-INV`, retained in core | 22 |
| historical relocated | 0 |
| narrower HOLD | 0 |
| promoted new `INV-*` / `OBL-*` | 0 |

## Resolved Rows

| Entry | Outcome | Binding IDs | Authority / rationale |
|---|---|---|---|
| `WB12-RECONCILIATION-COUPLING-ADDENDUM` | map-in-core | `INV-SUBHYD-009`, `INV-SUBHYD-021`, `INV-SUBHYD-022` | `Qd` closure handoff, same-pass WB19 flux handoff, and carryover precedence are existing SUBHYD authority. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | map-in-core | `INV-SUBHYD-009`, `INV-SUBHYD-021` | WB13 subsurface/drainage publication maps to `Qd` coupling semantics and flux-authoritative same-pass handoff. |
| `HPHYS0203-SUBSURFACE-WB13-ROBUSTNESS-VALIDATION-ADDENDUM` | map-in-core | `INV-SUBHYD-009`, `INV-SUBHYD-014`, `INV-SUBHYD-021` | Finite/non-negative publication, no-surrogate WB13 subsurface checks, and same-pass handoff are exposed; percolation-owned `Dp` context stays core narrative. |
| `HPHYS0234-WB13-SUBSURFACE-FLUX-AUTHORITY-ANTI-SHADOW-ADDENDUM` | map-in-core | `INV-SUBHYD-021` | WB13 `q`/`Qdd`/`Qd` flux-over-state anti-shadow authority is exposed by the WB19->WB12/WB13 handoff invariant. |
| `HPHYS0208-COUPLED-SUBSURFACE-RESIDUAL-CLOSURE-ADDENDUM` | map-in-core | `INV-SUBHYD-012`, `INV-SUBHYD-014`, `INV-SUBHYD-019` | WB19 layer-aware execution, fail-closed domain checks, and FC/WP + COCA water-yield lineage are existing SUBHYD authority. |
| `HPHYS0218-WB19-DRFC-THRESHOLD-LINEAGE-ADDENDUM` | map-in-core | `INV-SUBHYD-012`, `INV-SUBHYD-014`, `INV-SUBHYD-019`, `INV-SUBHYD-025`, `INV-SUBHYD-026` | `drfc_i`, `coca`, FC/WP + COCA coupling, frozen floors, and daily lane threshold semantics are exposed by existing invariants. |
| `HPHYS0221-WB19-WATER-YIELD-AND-SATURATED-DEPTH-COUPLING-ADDENDUM` | map-in-core | `INV-SUBHYD-015`, `INV-SUBHYD-019`, `INV-SUBHYD-024` | `solwpv` saturated-depth mutation, `avfca`/`watyld` theta lineage, and HPHYS0247 layer selection are existing authority. |
| `HPHYS0222-WB19-SOLWPV-BRANCH-AUTHORITY-CORRECTION-ADDENDUM` | map-in-core | `INV-SUBHYD-015`, `INV-SUBHYD-024` | `solwpv` mutation and saturated-layer selection were already mechanically mapped by SCSTRUCT06 and remain core-resident. |
| `HPHYS0224-WB19-REALIZED-WITHDRAWAL-SOIL-WATER-CAP-ADDENDUM` | map-in-core | `INV-SUBHYD-016` | Realized withdrawal cap and Level-4 suite linkage are exposed by `INV-SUBHYD-016`. |
| `HPHYS0225-WB19-LAYER-POOL-AVAILABLE-CAP-AUTHORITY-ADDENDUM` | map-in-core | `INV-SUBHYD-017` | Layer-pool available-cap authority and Level-4 suite linkage are exposed by `INV-SUBHYD-017`. |
| `HPHYS0226-WB19-LATERAL-SATURATED-THICKNESS-RESPONSE-ADDENDUM` | map-in-core | `INV-SUBHYD-018` | Saturated-thickness response authority and Level-4 suite linkage are exposed by `INV-SUBHYD-018`. |
| `HPHYS0227-WB19-FCWP-COCA-WATER-YIELD-COUPLING-ADDENDUM` | map-in-core | `INV-SUBHYD-019` | FC/WP + COCA water-yield coupling and Level-4 suite linkage are exposed by `INV-SUBHYD-019`. |
| `HPHYS0238-WB19-HOURLY-ITERATIVE-LATERALDRAINAGE-ADDENDUM` | map-in-core | `INV-SUBHYD-020` | Hourly per-substep recomputation and accumulated `q`/`Qdd` publication are exposed by `INV-SUBHYD-020`. |
| `HPHYS0239-WB19-WB12WB13-HANDOFF-ORDERING-ADDENDUM` | map-in-core | `INV-SUBHYD-021`, `INV-SUBHYD-023` | Same-pass handoff and anti-shadow semantics map to `INV-SUBHYD-021`; hourly drainage/lateral order maps to `INV-SUBHYD-023`. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-HANDOFF-ADDENDUM` | map-in-core | `INV-SUBHYD-022` | Carryover precedence, anti-shadow semantics, and malformed-boundary validation are exposed by `INV-SUBHYD-022`. |
| `HPHYS0242-HOURLY-DRAINAGELATERALSATURATION-TAIL-ADDENDUM` | map-in-core | `INV-SUBHYD-023` | Hourly tail ordering, final `Qd`, `ui_LfCrf`, and `ui_SCrunf` obligations are exposed by `INV-SUBHYD-023`. |
| `HPHYS0247-WB19-BASELINE-SATURATED-ZONE-CAPACITY-ADDENDUM` | map-in-core | `INV-SUBHYD-024`, `INV-SUBHYD-025`, `INV-SUBHYD-027` | Hourly `meblfc`/`fffx`/legacy multiplier authority, frozen-floor interaction, and modern hourly conductivity lineage are existing authority. |
| `HPHYS0252-WB19-FROZEN-ADJUSTED-LATERAL-STORAGE-ADDENDUM` | map-in-core | `INV-SUBHYD-024`, `INV-SUBHYD-025` | Frozen-adjusted `fzdrfc` floors map to `INV-SUBHYD-025`; unfrozen `drfc` conductivity weighting remains under `INV-SUBHYD-024`. |
| `HPHYS0256-WB19-DAILY-LATERAL-LANE-BRANCH-ADDENDUM` | map-in-core | `INV-SUBHYD-026` | Daily-vs-hourly lane selection and daily branch conductivity semantics are exposed by `INV-SUBHYD-026`. |
| `HPHYS0257-WB19-HOURLY-HORIZONTAL-CONDUCTIVITY-ADDENDUM` | map-in-core | `INV-SUBHYD-027` | Modern hourly `ui_ssh` / `wb19_lateral_ssh_####` lineage is exposed by `INV-SUBHYD-027`. |
| `HPHYS0258-WB19-HOURLY-CAPWITHDRAWAL-PUBLICATION-ADDENDUM` | map-in-core | `INV-SUBHYD-028` | Potential/target/`tdvv`/realized-withdrawal diagnostics and realized publication authority are exposed by `INV-SUBHYD-028`. |
| `HPHYS0259-WB19-TRACE-LOCALIZATION-ADDENDUM` | map-in-core | `INV-SUBHYD-029`, `INV-SUBHYD-030`, `INV-SUBHYD-031` | Trace serialization and same-surface identity checks map to `INV-SUBHYD-029`; active-zone and threshold-lineage evidence gates map to `INV-SUBHYD-030` and `INV-SUBHYD-031`. |

## Boundary Confirmation

All resolved rows remain core-resident because they carry active constitutive,
guard, or test-vector details. No sidecar relocation was performed.
