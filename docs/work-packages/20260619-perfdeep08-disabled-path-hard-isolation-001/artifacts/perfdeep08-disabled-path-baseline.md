# PERFDEEP08 Disabled-Path Baseline

Status: complete.
Evidence mode: Static/Ran.

## Starting Point

HEAD at execution start:

```text
93d7e90644864ae3f4b6b14cb04d8d307b39631b
```

PERFDEEP07 retained baseline:

- `685.85 s`, RSS `229004 KB`;
- dense-absent indexed bypass plus `HashMap` hot tables;
- still above P0 threshold `<= 676.67 s`.

Historical anchors:

- PERFDEEP05 default-disabled: `701.95 s`;
- PERFDEEP01/PERFMIG01 reference: `669.97 s`.

## Command Shape

Ran the candidate with all PERFDEEP/trace env vars unset:

```text
/usr/bin/time -f "perfdeep08_hook_cache_rep1\t%e\t%M" env -u OPENWEPP_PERFDEEP02_FRAME_ISLAND -u OPENWEPP_PERFDEEP03_LANE_DENSE_STATE -u OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH -u OPENWEPP_INDEXED_SHADOW_REPORT_PATH -u OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH -u OPENWEPP_HPHYS0245_TRACE_PATH target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run --output-dir /tmp/perfdeep08/hook-cache/rep1/h2637_same --policy compat --legacy-sidecar-discovery
```

Candidate binary:

```text
5e5871904faa650572b9781eda070482bac5348ddfb8dcf99aeea0146d7d7a06  target/release/openwepp-cli-hill
3cbc0eed219dec96c251c01a3d3d9506b8b1d32222c5877e94604ccdcecf007a  target/release/openwepp-cli-hill.json
```

## Candidate Result

```text
perfdeep08_hook_cache_rep1  691.93  229444
```

The candidate was slower than PERFDEEP07 and above the threshold, so no
three-run gate was started.
