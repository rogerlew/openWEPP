# HILLBENCH01 Disposition

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Decision
- GO

## Static
- Package objective achieved:
  - release benchmark lane established for single-OFE and multi-OFE hillslope
    runs,
  - baseline comparison vs `wepp_260430_hill` captured with repeatable harness,
  - scoped optimization edits landed in `openwepp-runner` runtime/CLI path,
  - before/after performance evidence recorded.
- Optimization outcome:
  - openWEPP median wall-time improved on both lanes (`-14.44%`, `-10.70%`).
- Residual gap:
  - openWEPP remains slower than baseline on both lanes post-optimization;
    additional waves are required for deeper runtime parity.

## Ran
- Benchmark harness executed pre and post optimization with persisted JSON
  artifacts.
- Required gate stack passed (`fmt`, `clippy`, workspace tests, `deny`).
