# Artifacts

Evidence for the `mn_corn_h4` day-792 raw-hydrograph numerics package.

Bulk rerun directories live under `artifacts/raw-hydrograph-numerics-runs/`
and are ignored. Committed evidence is limited to compact summaries, hashes,
command provenance, mechanism attribution, review, verification, gates, and
disposition artifacts.

Replay requires running the package-local ladder harness first, then the
analyzer:

```bash
OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \
  .venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/run_raw_hydrograph_numerics_ladder.py \
  --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625

.venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/analyze_raw_hydrograph_numerics.py
```
