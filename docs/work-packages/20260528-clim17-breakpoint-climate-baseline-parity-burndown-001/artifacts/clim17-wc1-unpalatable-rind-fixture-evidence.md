# CLIM17 WC1 Unpalatable-Rind Fixture Evidence

Status: complete  
Evidence mode: Static + Ran  
Date: 2026-05-28

## Static
- Corpus anchor path used:
  `/wc1/runs/un/unpalatable-rind/wepp/runs/p1.cli`
- Fixture extracted for contract vectors:
  `tests/fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli`
- Baseline parity anchors reviewed:
  - `/workdir/wepp-forest_260430_baseline/src/stmget.for:241-257`
  - `/workdir/wepp-forest_260430_baseline/src/brkpt.for:61-117`

## Ran

1. Breakpoint-cardinality prevalence scan (`p1.cli`):
   - Command:
     `awk 'BEGIN{...} ... END{printf ...}' /wc1/runs/un/unpalatable-rind/wepp/runs/p1.cli`
   - Output:
     - `records=1461`
     - `zero_nbrkpt=699`
     - `max_nbrkpt=19`

2. Baseline dry-day branch inspection:
   - Command:
     `nl -ba /workdir/wepp-forest_260430_baseline/src/stmget.for | sed -n '220,320p'`
   - Verified behavior:
     - `nbrkpt.gt.0` calls `brkpt`
     - `else` branch sets `rain=0`, `ninten=0`, `prcp=0`

3. Baseline breakpoint transform inspection:
   - Command:
     `nl -ba /workdir/wepp-forest_260430_baseline/src/brkpt.for | sed -n '50,170p'`
   - Verified behavior:
     - for `nbrkpt>0`, computes elapsed `timem`, `intsty`, `stmdur`, `mxint`,
       and terminal `intsty(nbrkpt)=0`, `prcp=pptcum(nbrkpt)`.
