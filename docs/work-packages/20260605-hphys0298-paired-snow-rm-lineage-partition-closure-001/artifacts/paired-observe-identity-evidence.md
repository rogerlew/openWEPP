# Paired Observe Identity Evidence

Status: complete

Evidence mode: ran

Ran:

```text
.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z --skip-full-suite --skip-targeted-traces
```

Result:

- Release binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`.
- Observe binary: `/tmp/hphys0298_wepp_forest_obs/src/wepp_hill`.
- Three lanes were executed for H1/H7/H39: pinned release without observe, instrumented observe-off, and instrumented observe-on.
- H1/H7/H39 release-to-observe-off WAT outputs were bit-identical.
- H1/H7/H39 observe-off-to-observe-on WAT outputs were bit-identical.
- H1/H7/H39 stored partition context matched target-window rows within `0.011 mm`; H1 had a `0.01 mm` storage-only text/partition rounding delta and exact `RM`/`Q`/`Snow-Water` identity.
- Details: `artifacts/baseline-observe-identity.md` and `artifacts/baseline-observe-identity.json`.
