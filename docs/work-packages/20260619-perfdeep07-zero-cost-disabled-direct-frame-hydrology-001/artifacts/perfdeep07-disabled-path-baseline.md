# PERFDEEP07 Disabled-Path Baseline

Status: HOLD.
Evidence mode: Static/Ran.

## Command Shape

Release binary:

`target/release/openwepp-cli-hill`

Final retained timing binary:

`beae925662febe47a741ce9e9e5bdd905f088a0601c70c8f4967b04c912b3c09`

All timing probes used:

```bash
/usr/bin/time -f '<label>\t%e\t%M' \
  env -u OPENWEPP_PERFDEEP02_FRAME_ISLAND \
      -u OPENWEPP_PERFDEEP03_LANE_DENSE_STATE \
      -u OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH \
      -u OPENWEPP_INDEXED_SHADOW_REPORT_PATH \
      -u OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH \
      -u OPENWEPP_HPHYS0245_TRACE_PATH \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file <rep runfile> \
  --output-dir <rep manifest dir> \
  --policy compat \
  --legacy-sidecar-discovery
```

## Timing Probes

| Candidate | Result |
|---|---:|
| Plain no-indexed overreach | `753.38 s`, RSS `229392 KB` |
| Plain no-indexed after logical-map bypass | `755.48 s`, RSS `228612 KB` |
| Dense-absent indexed bypass | `692.52 s`, RSS `228148 KB` |
| Dense-absent indexed bypass + `HashMap` hot tables | `685.85 s`, RSS `229004 KB` |
| Forced indexed-surface rebuild | `1035.90 s`, RSS `228416 KB` |
| Indexed-surface report propagation | `1054.71 s`, RSS `228416 KB` |
| Hot-absent bypass variant | `688.54 s`, RSS `228640 KB` |

The retained candidate is dense-absent indexed bypass plus `HashMap` hot
tables. It improved over PERFDEEP05 default-disabled `701.95 s`, but did not
meet the required median threshold of `<= 676.67 s`. The three-run P0 median
gate was not completed because all viable single-run candidates remained above
the threshold and direct-frame work is blocked until P0 passes.

## Identity Evidence

Compared `/tmp/perfdeep07/hash-hot/rep1/h2637_same` against
`/tmp/perfdeep05/default/h2637_same` for protected outputs:

- HBP byte-identical:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`.
- WAT byte-identical:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`.
- PASS Arrow schema/table equality: pass, `12419` rows each.
- WAT Arrow schema/table equality: pass, `235961` rows each.
- Loss and plot sidecar text differed only by `run_name`.
