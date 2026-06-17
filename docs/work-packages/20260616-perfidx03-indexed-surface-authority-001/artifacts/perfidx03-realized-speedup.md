# PERFIDX03 Realized Speedup

Status: FAIL 2026-06-17
Evidence mode: **Ran**

## Result

The active indexed-authority lane regressed OFE5 wall-clock. The regression is
consistent across three interleaved samples:

- Baseline mean: `27.01s`.
- Active indexed authority mean: `38.34s`.
- Regression: about `+41.9%` wall-clock.

The root cause is architectural, not noise: the attempted live path cloned the
sparse indexed surface, then exported it back to full `BTreeMap` state/flux maps
for the existing kernel seam on every lane/day. That added export work overwhelms
the sparse clone win in the current Stage 3 compatibility design.

## Interleaved OFE5 Timing

Command family:

```text
/usr/bin/time -f "PERFIDX03_OFE5_TIME ..." \
  <baseline-or-current-openwepp-cli-hill> \
  --run-dir /tmp/perfho01/run-dirs/ofe5 \
  --run-file /tmp/perfidx03/speed/runfiles/ofe5_<variant>_run<N>.run \
  --output-dir /tmp/perfidx03/speed/<variant>/ofe5_run<N>_manifest \
  --policy compat
```

Raw results:

```text
PERFIDX03_OFE5_TIME variant=baseline run=1 elapsed=26.75 user=26.70 sys=0.02 maxrss_kb=25728
PERFIDX03_OFE5_TIME variant=current run=1 elapsed=38.42 user=38.36 sys=0.05 maxrss_kb=26724
PERFIDX03_OFE5_TIME variant=baseline run=2 elapsed=26.99 user=26.96 sys=0.02 maxrss_kb=26112
PERFIDX03_OFE5_TIME variant=current run=2 elapsed=38.30 user=38.25 sys=0.04 maxrss_kb=26644
PERFIDX03_OFE5_TIME variant=baseline run=3 elapsed=27.29 user=27.25 sys=0.03 maxrss_kb=25728
PERFIDX03_OFE5_TIME variant=current run=3 elapsed=38.30 user=38.25 sys=0.04 maxrss_kb=26284
```

Same-run-name OFE5 confirmation:

```text
PERFIDX03_OFE5_BITID_TIME variant=baseline elapsed=27.26 user=27.23 sys=0.02 maxrss_kb=25728
PERFIDX03_OFE5_BITID_TIME variant=current elapsed=38.22 user=38.16 sys=0.04 maxrss_kb=27044
```

## No-Flip Rollback Timing

The production activation call was disabled before disposition. With the indexed
authority not activated, the current tree returns to baseline-range timing:

```text
PERFIDX03_OFE5_NOFLIP_TIME elapsed=26.80 user=26.76 sys=0.03 maxrss_kb=26136
```

## Disposition

The Stage 3 authority flip does not meet the realized speedup requirement. It is
held until the kernel seam can consume an indexed/cached representation without
paying a full `BTreeMap` export on the hot path.
