# Implementation and Test Evidence

Status: complete

Evidence mode: ran

Static:

- Added package-local runner `artifacts/hphys0298_paired_lineage_partition.py`.
- Added diagnostic-only baseline patch artifact `artifacts/baseline-observe-instrumentation.patch`.
- No openWEPP production physics files were changed in this package.
- Retrospective correction: after `review_claude_hrsnow_unit_artifact.md`, the
  runner intentionally fails closed when invoked because its historical
  `hrsnow` pairing uses `snow_hourly_snowfall_water_equiv_sum_m`. The pass
  result below is pre-retrospective historical evidence, not current rerun
  behavior or migration authority.

Ran:

```text
git -C /workdir/wepp-forest_260430_baseline worktree add --detach /tmp/hphys0298_wepp_forest_obs dac3c950d8b16cc73774bf5ce2e7e11f80baac70
make clean && make COMPILER=gfortran wepp_hill
.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z
```

Result:

- Baseline diagnostic build passed.
- Pre-retrospective HPHYS0298 runner passed with exit code `0`; current runner
  behavior is expected-fail exit code `2` until the harness is corrected to
  pair canonical `hrsnow` with `snow_hourly_snowfall_depth_sum_m`.
- Baseline observe identity passed for H1/H7/H39.
- Full H1..H39 semantic metrics were written to `artifacts/full-39-suite-metrics.md`.
- All nine H1/H7/H39 target windows were classified in `artifacts/paired-lineage-summary.md` and `artifacts/paired-lineage-ledger.json`.
- Current validation is recorded in `artifacts/gate-results.md`: the
  unit-pairing guard rejects the historical depth-vs-water-equivalent evidence
  and directs continuation to HPHYS0299 corrected depth-vs-depth authority.
