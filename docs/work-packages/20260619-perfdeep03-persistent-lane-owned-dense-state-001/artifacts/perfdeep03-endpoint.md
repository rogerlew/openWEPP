# PERFDEEP03 Endpoint

Evidence class: Ran.

## Verdict

`FAIL - endpoint did not beat PERFDEEP01`.

The load-bearing PERFDEEP03 gate required the opt-in lane dense path to beat the
PERFDEEP01 H2637 endpoint reference:

```text
669.97 s
```

The completed opt-in lane dense H2637 run measured:

```text
1147.96 s, 229580 KB
```

That is `477.99 s` slower than PERFDEEP01 and `1.713x` the reference elapsed
time. PERFDEEP03 therefore triggers the package falsification boundary and does
not authorize default activation or island expansion.

## Opt-In Endpoint Command

Ran:

```text
env OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1 \
  /usr/bin/time -f "h2637_perfdeep03_lane_dense_hot_first\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/runfiles/perfdeep03-h2637.run \
  --output-dir /tmp/perfdeep03/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
h2637_perfdeep03_lane_dense_hot_first 1147.96 229580
```

## Diagnostic Roundtrip Run

Ran:

```text
env OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1 \
  OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH=/tmp/perfdeep03/frame_roundtrip/h2637_final.jsonl \
  /usr/bin/time -f "h2637_perfdeep03_final_lane_dense_roundtrip\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/runfiles/perfdeep03-h2637.run \
  --output-dir /tmp/perfdeep03/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
h2637_perfdeep03_final_lane_dense_roundtrip 3563.75 234716
```

The diagnostic run is not used as the endpoint-performance verdict because the
JSONL roundtrip hook is intentionally heavy.

Sidecar warning observed:

```text
MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope.
```

This is an existing scoped warning and not a PERFDEEP03 performance verdict.

## Default-Disabled Endpoint Check

Ran without `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`:

```text
h2637_perfdeep03_default_disabled 697.36 227988
h2637_perfdeep03_default_cached_gate 707.80 227672
```

Default identity passed, but default endpoint flatness was not proven against
the `669.97 s` reference. The dense path remains disabled by default; no default
activation is authorized.
