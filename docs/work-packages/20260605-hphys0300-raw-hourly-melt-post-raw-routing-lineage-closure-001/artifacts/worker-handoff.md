# Worker Handoff

Status: complete

Evidence mode: static + ran

Static:

- Package:
  `docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/`
- Final status: `executed-hold`.
- No production kernel files were edited.
- Contract authority added:
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-031` and
  `SC-WATBAL-001#INV-WATBAL-075`.

Ran:

- Diagnostic runner:
  `.venv/bin/python docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/hphys0300_raw_post_raw_lineage.py --run-root /tmp/hphys0300_full_20260605T155527Z --artifact-dir docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts`
- Focused and workspace gates are recorded in `gate-results.md`.

Primary Artifacts:

- `raw-post-raw-lineage-ledger.json`
- `raw-post-raw-lineage-summary.md`
- `corrected-partition-ledger.json`
- `baseline-observe-identity.md`
- `full-39-suite-metrics.md`
- `full-39-suite-summary.json`
- `review-disposition.md`
- `disposition.md`

Continuation Route:

- Use a forcing-function package, not a ninth pure diagnostic. Fix H39
  first-2013 corrected-depth hourly forcing when its source-line root cause is
  confirmed, and instrument paired baseline/openWEPP term/state lineage for
  the seven raw-hourly-melt windows.
- Required raw/post-raw symbols: `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`,
  `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`, and `densgt`.
- Once term/state evidence isolates a named producer source with units and
  source-line provenance, implement the baseline-authoritative producer
  correction in that package or record the concrete blocking invariant; do not
  route the same isolated source into another diagnostic-only package.
- Keep H7 first-2013 as post-raw/routed-melt hold unless term/state evidence
  proves otherwise.
- Keep H39 first-2013 as a corrected-depth hourly-forcing seam, not generic
  raw-melt closure.
- Keep WB17/WB18/WB19/WB13 compensation prohibited.
