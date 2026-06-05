# Implementation/Test Evidence

Status: complete

Evidence mode: ran

Ran initial full diagnostic command:

```text
RUN_ROOT=/tmp/hphys0299_full_$(date -u +%Y%m%dT%H%M%SZ)
echo "$RUN_ROOT" > /tmp/hphys0299_run_root.txt
python3 docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/hphys0299_corrected_partition.py --run-root "$RUN_ROOT"
```

Result: partial failure after full-suite and targeted trace phases completed.
Failure was at baseline identity partition comparison because system Python
lacked `pyarrow`:

```text
ModuleNotFoundError: No module named 'pyarrow'
```

Ran resumed command:

```text
.venv/bin/python docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/hphys0299_corrected_partition.py --run-root /tmp/hphys0299_full_20260605T101220Z --skip-full-suite --skip-targeted-traces
```

Context: initial invocation with system `python3` completed full H1..H39 suite
and targeted H1/H7/H39 traces, then failed during baseline identity because
system Python lacked `pyarrow`. The resumed invocation used `.venv/bin/python`,
which has `pyarrow`, and skipped already-completed full-suite/traces for the
same run root.

Ran full-suite and trace evidence:

- Run root: `/tmp/hphys0299_full_20260605T101220Z`
- Full H1..H39 metrics: `artifacts/full-39-suite-metrics.md`
- Full H1..H39 summary JSON: `artifacts/full-39-suite-summary.json`
- Full H1..H39 hillslope status: `artifacts/full-39-hillslope-batch-status.tsv`
- Full H1..H39 semantic status: `artifacts/full-39-semantic-status.tsv`
- Corrected partition ledger: `artifacts/corrected-partition-ledger.md`
- Unit provenance audit: `artifacts/unit-provenance-audit.md`
- Targeted trace status: `artifacts/target-trace-status.tsv`
- Baseline observe status: `artifacts/baseline-observe-status.tsv`
- Baseline observe identity: H1/H7/H39 release, observe-off, observe-on all
  returned `rc=0`, bit identity `True`, semantic identity `True`, and partition
  identity `True`.
- Targeted traces: H1, H7, and H39 returned `rc=0`.

Corrected verdict summary:

- `OPENWEPP-DEFECTIVE`: `9` windows.
- First cut-points: `raw-hourly-melt=7`, `negative-melt-correction=1`,
  `hourly-forcing=1`.
- Canonical `hrsnow` provenance: all rows use
  `snow_hourly_snowfall_depth_sum_m`; no row maps `hrsnow` to
  `snow_hourly_snowfall_water_equiv_sum_m`.

Interpretation:

- HPHYS0298's all-window hourly-forcing verdict was over-broad due to the
  depth-vs-water-equivalent seam.
- Seven windows now route to raw hourly melt, one to corrected negative-melt
  authority, and only H39 first-2013 remains an hourly-forcing producer defect.
- Downstream WB17/WB18/WB19/WB13 compensation remains unauthorized.
