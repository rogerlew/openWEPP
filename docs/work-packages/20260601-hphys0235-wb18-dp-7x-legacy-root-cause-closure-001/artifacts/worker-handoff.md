# HPHYS0235 Worker Handoff

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Immediate Next Actions

1. Open follow-on implementation package (`hphys0236`) to migrate hourly WB18
   percolation to legacy-authoritative iterative execution shape:
   - explicit `24`-substep loop under hourly lane,
   - per-substep recomputation of per-layer routing terms,
   - accumulated bottom-layer seepage publication to `D`/`Pe`.
2. Add contract-derived executable vectors for hourly iterative semantics:
   - regression guard against divisor-only single-pass behavior,
   - targeted `H1` transient acceptance checks.
3. Rerun `unpalatable-rind` (`H1..H39`) and publish monitored-column deltas
   vs HPHYS0234/0235 baseline summaries.
4. Run workspace gates in the implementation package (`fmt`, `clippy`, `test`,
   `deny`) before disposition.

## Evidence Anchors

- Hourly manifest: `/tmp/hphys0235_probe/out_hourly/openwepp_hillslope_run_manifest.json`
- Daily manifest: `/tmp/hphys0235_probe/out_daily/openwepp_hillslope_run_manifest.json`
- Daily probe output: `/tmp/hphys0235_probe/hillslope_output/H1.wat.parquet`
- Hourly reference output: `/tmp/hphys0234_20260601T215019Z/parity/hillslope_output/H1.wat.parquet`
- Baseline comparator: `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`
