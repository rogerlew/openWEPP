# HPHYS0248 Worker Handoff

Status: hold

Evidence mode: Static + Ran

Static:
- Continue from HPHYS0248 only if the scope is to inspect this package’s
  residual evidence. Open a follow-on work package for additional production
  migration beyond the touched WB18 bottom-percolation surface.

Ran:
- Corrected H39 early-season WB18 `Dp`/`Pe` scale by porting baseline hourly
  bottom restrictive-layer lineage:
  - `ui_bdrkth` + `kslast` thickness-weighted effective conductivity.
  - hourly bottom-layer `fx=1` behavior from legacy `meblfc`.
  - typed fail-closed guards for missing, non-finite, or non-positive
    `ui_bdrkth` when restrictive hourly bottom seepage is active.
- Primary run evidence:
  `/tmp/hphys0248_20260602T114714Z_final`.
- H39 first 10 days: baseline `Dp=0.240000 mm/day`, candidate
  `Dp=0.246960 mm/day`, residual `+0.006960 mm/day`.
- Full `H1..H39` runtime and comparator report generation completed
  (`39/39`), but semantic pass remains `0/39`.
- Remaining priority order:
  1. WB17 `Ep`/`Es` partition.
  2. Snowmelt/runoff timing (`Snow-Water`, `RM`, `Q`).
  3. Aggregate storage (`Total-Soil`, `SoilWaterTotal`) after ET/snow timing.
  4. WB19 `latqcc`, especially H33, after WB18 overdrainage is corrected.
