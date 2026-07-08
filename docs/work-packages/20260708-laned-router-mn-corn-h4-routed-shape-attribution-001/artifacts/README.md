# Artifacts

Evidence for the `mn_corn_h4` routed-shape attribution package.

Bulk rerun directories live under `artifacts/shape-attribution-runs/` and are
ignored. Committed evidence is limited to compact summaries, hashes, command
logs, attribution tables, review, verification, gates, and disposition
artifacts.

Replay sequence:

1. Run `run_shape_attribution_ladder.py` with
   `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`.
2. Run `analyze_day792_attribution.py` to regenerate
   `day792-attribution.json` and `day792-attribution.md` from the ignored raw
   trace trees.
