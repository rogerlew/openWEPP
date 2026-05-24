# simimpl02 routine contract invariant crosswalk

Status: phase-c-complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Crosswalk authority limited to canonical contracts listed in SIMIMPL02
  dependencies:
  - `SC-WATBAL-001`
  - `SC-SYSTEM-001`
  - `SC-RUNOFFPART-001`
  - `SC-INFILE-WEPPUI-001`
- Crosswalk is routine-family based: it maps legacy routine families to
  contract surfaces and invariant/obligation sets consumed by SIMIMPL03+.

## Ran
- Contract sections/intake verified via direct file probes for invariant, guard,
  obligation, and gap-register sections.
- openWEPP evidence anchors probed to align owner surfaces:
  - runner entry/publication path
  - orchestrator scheduler/kernel execution lanes
  - climate runtime adapter
  - `wepp_ui` parser contract implementation surface.

## Family crosswalk
| Legacy routine family | Primary contract authority | Invariant/obligation linkage | Owner-surface closure implication |
|---|---|---|---|
| `watbal`, `watbal_hourly` | `SC-WATBAL-001`, `SC-INFILE-WEPPUI-001`, `SC-SYSTEM-001` | `INV-WATBAL-001/009/012/014/017`, `INV-SYSTEM-011/012/014/017`, `G-WUI-004` | Runner/runtime lane selection and strict replay publication remain unresolved and block production closure until SIMIMPL03-07. |
| `hydout`, `watbalprint` | `SC-WATBAL-001`, `SC-SYSTEM-001` | `WB13` addendum (`HKERNEL-WB13-HWAT-*` guards), `INV-SYSTEM-012/014/017` | Output surface ownership must migrate from projection-first publication to simulation-owned emission (`simimpl06`). |
| `contin` orchestration shell | `SC-SYSTEM-001`, `SC-WATBAL-001` | `INV-SYSTEM-011`, `INV-WATBAL-011` | Phase-order closure exists in orchestrator abstractions but is not wired through production runner (`simimpl05`). |
| `soil`, `frsoil`, `winter` couplings | `SC-WATBAL-001`, `SC-RUNOFFPART-001`, `SC-INFILE-WEPPUI-001` | `INV-WATBAL-007/013`, `INV-RUNOFFPART-009`, `G-WUI-001..007` | Parser/runtime coupling is partial; full frozen-soil/snow and hourly closure deferred (`simimpl10`). |
| `evap`, `evappm`, `drain`, `decomp` | `SC-WATBAL-001` (WB17/WB18/WB19), `SC-SYSTEM-001` | `INV-WATBAL-009/010`, `OBL-WATBAL-P-003/004` | Kernel-phase abstractions are mapped in openWEPP and serve as closure anchors for gap families. |
| `stmget` lineage | `SC-WATBAL-001`, `SC-SYSTEM-001` | `INV-SYSTEM-010` governance linkage, climate breakpoint guard posture | Climate adapter seam is present; downstream closure depends on full runtime execution ownership, not parser absence. |
| `runoff/erosion/channel/impoundment` deferred families (`route`, `sloss`, `sed*`, `chn*`, `wsh*`, `imp*`) | `SC-SYSTEM-001`, `SC-RUNOFFPART-001` (+ companion contracts in registry gap notes) | `INV-SYSTEM-001..010`, `INV-RUNOFFPART-001..011`, `GAP-SYSTEM-001`, `GAP-RUNOFFPART-003` | Explicitly deferred from SIMIMPL02 hillslope closure map; retained as `deferred` rows for later watershed/erosion wave intake. |
| unresolved legacy callees (`imppol`, `imppow`, `impris`) | `SC-SYSTEM-001` (deferred watershed/impoundment lane governance) | `INV-SYSTEM-001/007/008` boundary continuity implications | Require separate provenance/discovery package before promotable system closure; not on current hillslope critical path. |

## Downstream contract-first obligations recorded
1. SIMIMPL03 must ratify required contract amendments before code edits.
2. SIMIMPL04 must add contract-derived tests and pre-implementation gate
   evidence before production wiring work.
3. SIMIMPL05+ may modify runner/orchestrator/output code only after (1) and (2)
   are complete.
