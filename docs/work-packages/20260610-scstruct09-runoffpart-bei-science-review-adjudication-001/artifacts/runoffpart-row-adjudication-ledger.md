# SCSTRUCT09 RUNOFFPART Row Adjudication Ledger

Evidence: Static
Date: 2026-06-11
Target: `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`

## Summary

SCSTRUCT09 adjudicated the 13 rows routed from SCSTRUCT08 and refined the two
mechanically mapped rows. All 15 `SC-RUNOFFPART-001` Binding Exposure Index rows
now map to existing `INV-RUNOFFPART-*` authority. No row required promotion,
historical relocation, or a narrower HOLD.

| Outcome | Count |
|---|---:|
| `maps-to-existing-INV`, retained in core | 15 |
| historical relocated | 0 |
| narrower HOLD | 0 |
| promoted new `INV-*` / `OBL-*` | 0 |

## Resolved Rows

| Entry | Outcome | Binding IDs | Authority / rationale |
|---|---|---|---|
| `EROD12-CROSS-DOMAIN-OWNERSHIP-AND-GUARD-CLOSURE-ADDENDUM` | map-in-core | `INV-RUNOFFPART-007`, `INV-RUNOFFPART-008`, `INV-RUNOFFPART-009`, `INV-RUNOFFPART-011` | Runoff/peak-duration and multi-OFE producer ownership maps to existing case, aggregation, coupling, and forward-lane invariants; cross-contract consumer context remains active core detail. |
| `WB12-RUNOFF-RECONCILIATION-ADDENDUM` | map-in-core | `INV-RUNOFFPART-001`, `INV-RUNOFFPART-009`, `INV-RUNOFFPART-011`, `INV-RUNOFFPART-012` | WB12 closure equation, daily coupling, forward-lane closure-delta, and carryover precedence are existing RUNOFFPART authority. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-009`, `INV-RUNOFFPART-019`, `INV-RUNOFFPART-020`, `INV-RUNOFFPART-021` | WB13 runoff/runon and `RM` publication maps to downstream coupling, routed melt, post-winter rain, and snow publication lifecycle invariants. |
| `WB14-INFILTRATION-AND-SUBDAILY-HYETOGRAPH-KERNEL-AUTHORITY-ADDENDUM` | map-in-core | `INV-RUNOFFPART-001`, `INV-RUNOFFPART-002`, `INV-RUNOFFPART-004`, `INV-RUNOFFPART-011`, `INV-RUNOFFPART-012`, `INV-RUNOFFPART-015`, `INV-RUNOFFPART-016`, `INV-RUNOFFPART-017`, `INV-RUNOFFPART-027` | Hyetograph, Green-Ampt, runoff reconciliation, snow-liquid guard, same-pass infiltration handoff, carryover, and top-two-layer storage-limit semantics are existing invariants. |
| `WB15-CANOPY-INTERCEPTION-RUNTIME-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-001`, `INV-RUNOFFPART-002`, `INV-RUNOFFPART-009` | Interception-before-infiltration reduces liquid supply inside event partition, rainfall-excess, and downstream coupling invariants; plant/cap guard detail remains core. |
| `IRRIG10-IRRIGATION-RUNTIME-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-001`, `INV-RUNOFFPART-009`, `INV-RUNOFFPART-016` | Irrigation forcing participates in event closure and daily `Q` coupling; same-pass infiltration/runoff availability maps to `INV-RUNOFFPART-016`. |
| `CLIM05-SNOW-RUNTIME-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-001`, `INV-RUNOFFPART-015`, `INV-RUNOFFPART-017` | Signed snow liquid forcing maps to event closure and routed-melt infiltration forcing; projected snow-state guard posture maps to `INV-RUNOFFPART-017`. |
| `CLIM06-FROZEN-SOIL-RUNTIME-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-001`, `INV-RUNOFFPART-004`, `INV-RUNOFFPART-009` | Frozen infiltration-capacity consumption and domains are exposed by event closure, infiltration-domain, and downstream coupling invariants. |
| `WB16-PEAK-RUNOFF-KERNEL-ADDENDUM` | map-in-core | `INV-RUNOFFPART-005`, `INV-RUNOFFPART-009` | Peak/runoff-duration branch equations, near-zero behavior, and `m`/`ealpha` producer provenance are exposed by peak-discharge and coupling invariants. |
| `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | map-in-core | `INV-RUNOFFPART-009`, `INV-RUNOFFPART-011` | Typed production surfaces preserve required runoff boundary payloads and WB12/WB16 guard behavior exposed by coupling and forward-lane invariants. |
| `EROD13-WAVE-1-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-005`, `INV-RUNOFFPART-009` | Wave-1 mandatory `Q`, `peakro`, `watdur`, and branch surfaces map to peak-discharge and downstream coupling invariants. |
| `EROD14-WAVE-2-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | map-in-core | `INV-RUNOFFPART-007`, `INV-RUNOFFPART-008`, `INV-RUNOFFPART-009` | Wave-2 multi-OFE producer surfaces map to case-classification, aggregation, and coupling invariants. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-ADDENDUM` | map-in-core | `INV-RUNOFFPART-012` | Same-pass carryover precedence, republished carryover, anti-shadow behavior, and malformed flux rejection are exposed by `INV-RUNOFFPART-012`. |
| `HPHYS0241-MOFE-HOURLY-CARRY-ARRAY-RUNOFF-ADDENDUM` | map-in-core | `INV-RUNOFFPART-013` | Hourly upstream carry-array authority, aggregate anti-shadow, area-scaling provenance, and malformed-array rejection are exposed by `INV-RUNOFFPART-013`. |
| `HPHYS0242-SURFACE-SATURATION-RUNOFF-ADDBACK-ADDENDUM` | map-in-core | `INV-RUNOFFPART-014` | `surdra` addback, same-pass `Q` closure, hidden-storage prohibition, and current-array rejection are exposed by `INV-RUNOFFPART-014`. |

## Boundary Confirmation

All resolved rows remain core-resident because they carry active runtime, guard,
producer, vector, or cross-domain context. No sidecar relocation was performed.
The actual SCSTRUCT08 queue did not contain the HPHYS0296-0298 snow/`RM` arc
rows anticipated by the package background; those obligations were already
represented in existing `INV-RUNOFFPART-024..026`.
