# HPHYS0237 Worker Handoff

Status: completed  
Evidence mode: mixed (`Static` + `Ran`)

## Immediate Next Actions

1. Open follow-on implementation package for **Dispatch Group A**:
   WB19 iterative lateral + drainage substep migration.
2. In that package, amend `SC-SUBHYD-001`/`SC-WATBAL-001` for explicit hourly
   WB19 substep authority before code edits.
3. Add contract-derived tests proving:
   - hourly iterative recompute behavior,
   - non-regression against divisor-only or single-pass daily substitutions.
4. Open follow-on package for **Dispatch Group B** (phase ordering + runoff
   carryover) once Group A lands and rerun diagnostics are published.
5. Open follow-on package for **Dispatch Group C** (MOFE hourly carry arrays)
   after Group B ordering surfaces are settled.
6. Open follow-on package for **Dispatch Group D** (WB14/WB12 cadence and
   infiltration-observation ordering closure) to eliminate remaining daily
   single-pass authority gaps.

## Primary Inventory Artifact

- `artifacts/hphys0237-hourly-routine-inventory.md`
