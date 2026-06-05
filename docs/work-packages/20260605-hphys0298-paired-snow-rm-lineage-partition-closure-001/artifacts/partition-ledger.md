# Partition Ledger

Status: complete

Evidence mode: ran

Ran:

```text
.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z --skip-full-suite --skip-targeted-traces
```

Result:

- Markdown summary: `artifacts/paired-lineage-summary.md`.
- JSON ledger: `artifacts/paired-lineage-ledger.json`.
- All nine H1/H7/H39 target windows were classified as `OPENWEPP-DEFECTIVE`.
- First divergent cut-point: `hourly-forcing` for all nine windows.
- First divergent symbol: `hrsnow` for eight windows; `hrrain,hrsnow` for H39 2013 days `97-112`.
- `Q` remained closed in all nine windows; the defect source is upstream of downstream storage consumers.
- Verdict interpretation: this is a porting-fidelity defect against the
  unimpeached pinned-baseline precipitation-phase partition at
  `/workdir/wepp-forest_260430_baseline/src/winter.for:410-412`.
- The JSON ledger includes per-window `source_provenance` rows with canonical symbol, openWEPP symbol, unit, values, deltas, and source path/line references.

Continuation:

- Open a follow-on package for baseline-authoritative winter hourly snow/rain
  forcing partition migration of `winter.for:410-412` before returning to raw
  melt or downstream water-balance consumers.
