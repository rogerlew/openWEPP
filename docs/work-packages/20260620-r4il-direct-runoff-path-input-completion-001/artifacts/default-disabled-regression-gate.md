# R4I-L Default-Disabled Regression Gate

Status: complete.

Evidence class: Ran.

Required gate: H2637 default-disabled median `<= 676.67 s` with direct-runtime
and diagnostic environment variables unset.

## Runs

Each run used the release `openwepp-cli-hill` binary with these variables
unset:

- `OPENWEPP_PERFDEEP02_FRAME_ISLAND`
- `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`
- `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`
- `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`
- `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`
- `OPENWEPP_HPHYS0245_TRACE_PATH`

```text
r4il_h2637_default_rep1 646.47 228352
r4il_h2637_default_rep2 642.52 228324
r4il_h2637_default_rep3 640.20 228824
```

Median: `642.52 s`.

Gate: PASS. Median is below the `676.67 s` threshold.

Known warning retained from earlier runs: `MOFE01-MG-W-001` sidecar warning.
It does not change PASS row identity or the default-disabled regression result.
