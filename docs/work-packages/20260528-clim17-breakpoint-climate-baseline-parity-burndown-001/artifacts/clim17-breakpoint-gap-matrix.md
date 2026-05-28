# CLIM17 Breakpoint Gap Matrix

Status: complete  
Evidence mode: Static  
Date: 2026-05-28

## Gap Matrix

| Gap ID | Baseline authority | openWEPP observed behavior before CLIM17 | Classification | CLIM17 disposition |
|---|---|---|---|---|
| `CLIM17-GAP-001` | `/workdir/wepp-forest_260430_baseline/src/stmget.for:241-257` (`nbrkpt>0` routes through `brkpt`; `nbrkpt=0` sets `prcp=0`, `ninten=0`) | `openwepp-climate-runtime-adapter` returned `CLIM-RUNTIME-E-008` for empty breakpoint vectors, including `nbrkpt=0` dry days. | runtime + seam parity | closed |
| `CLIM17-GAP-002` | Same as above, plus `/wc1/runs/un/unpalatable-rind/wepp/runs/p1.cli` corpus prevalence | No explicit parser/runtime seam vector coverage for curated `ibrkpt=1`, `nbrkpt=0` days. | test gap | closed |
| `CLIM17-GAP-003` | Canonical SC authority requirement for kernel-adjacent runtime projection | `SC-CLIMATE-001` and `SC-INFILE-CLIMATE-001` lacked explicit breakpoint dry-day parity requirement text. | contract gap | closed |

## Static
- Baseline dry-day branch logic verified in `stmget.for` and breakpoint transform
  semantics verified in `brkpt.for`.
- Gap classification and closure mapping complete.

## Ran
- not-run
