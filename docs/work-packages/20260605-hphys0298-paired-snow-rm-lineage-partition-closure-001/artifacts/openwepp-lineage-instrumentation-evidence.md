# openWEPP Lineage Instrumentation Evidence

Status: complete

Evidence mode: ran

Static:

- HPHYS0298 reused the existing opt-in HPHYS0245/HPHYS0297 JSONL trace surface instead of adding production trace fields.
- Trace path was selected with `OPENWEPP_HPHYS0245_TRACE_PATH`.
- Trace length was selected with `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1800`.
- Target output files:
  - `/tmp/hphys0298_full_20260605T000000Z/hillslope_output/H1.hphys0298.trace.jsonl`
  - `/tmp/hphys0298_full_20260605T000000Z/hillslope_output/H7.hphys0298.trace.jsonl`
  - `/tmp/hphys0298_full_20260605T000000Z/hillslope_output/H39.hphys0298.trace.jsonl`

Ran:

```text
.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z
```

Result:

- Targeted openWEPP traces for H1/H7/H39 completed with `rc=0`.
- Trace status table: `/tmp/hphys0298_full_20260605T000000Z/reports/hphys0298_target_trace_status.tsv`.
