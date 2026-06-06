# Partition Ledger

Status: historical/superseded

Evidence mode: ran + static supersession

Static:

- Retrospective review `artifacts/review_claude_hrsnow_unit_artifact.md`
  found the historical `hrsnow` verdict paired baseline snowfall depth against
  openWEPP `snow_hourly_snowfall_water_equiv_sum_m`, a water-equivalent
  accounting surface.
- HPHYS0299 supersedes the HPHYS0298 all-window `hourly-forcing` migration
  inference with corrected depth-vs-depth `hrsnow` evidence.

Ran:

```text
.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z --skip-full-suite --skip-targeted-traces
```

Result:

- Markdown summary: `artifacts/paired-lineage-summary.md`.
- JSON ledger: `artifacts/paired-lineage-ledger.json`.
- Historical classifier output: all nine H1/H7/H39 target windows were
  classified as `OPENWEPP-DEFECTIVE`.
- Historical first divergent cut-point: `hourly-forcing` for all nine windows.
- Historical first divergent symbol: `hrsnow` for eight windows;
  `hrrain,hrsnow` for H39 2013 days `97-112`.
- `Q` remained closed in all nine windows; the defect source is upstream of downstream storage consumers.
- Verdict interpretation: superseded. The `hrsnow` comparison was
  depth-vs-water-equivalent and is non-authoritative for production migration.
- The JSON ledger includes per-window `source_provenance` rows with canonical symbol, openWEPP symbol, unit, values, deltas, and source path/line references.

Continuation:

- Do not open a winter hourly snow/rain forcing migration from HPHYS0298 alone.
  Continue from HPHYS0299 corrected depth-vs-depth `hrsnow` evidence and later
  residual localization artifacts.
