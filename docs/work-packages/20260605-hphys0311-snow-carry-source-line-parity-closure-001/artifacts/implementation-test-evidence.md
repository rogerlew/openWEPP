# Implementation/Test Evidence

Status: complete

Evidence mode: ran

Static:

- Implemented `artifacts/hphys0311_snow_carry_source_line_parity.py`.
- Runner fails closed on missing source-line and paired-state evidence.
- Runner writes source-line parity ledger, summary, method, and source-lineage
  artifacts.
- No production Rust kernel edit was made.

Ran:

- `.venv/bin/python` compiled
  `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/hphys0311_snow_carry_source_line_parity.py`
  to `/tmp/hphys0311_snow_carry_source_line_parity.pyc`
  passed.
- `.venv/bin/python docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/hphys0311_snow_carry_source_line_parity.py`
  generated HPHYS0311 artifacts.
- `jq` confirmed `7` groups, `58` represented HPHYS0309 rows, route counts
  `6/1` (`prior-year-terminal-state-hold`/`fixed-observe-precision-hold`),
  and `0` authorized production edits.

Post-review implementation repairs:

- Source-line requirements now cite the baseline carry, settling, writeback,
  sidecar seed, initialization, openWEPP runtime alias, hourly update, and
  `SC-INFILE-MANAGEMENT-001` authority lines.
- Day-1 inherited-state routing now requires both depth-delta and density-delta
  inheritance.
- H1 2013 settling routing now requires previous-hour depth/density states to
  be near-identical before using `fixed-observe-precision-hold`; otherwise it
  routes to `prior-hour-carry-state-hold`.
