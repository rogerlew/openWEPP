# HPHYS0235 Implementation and Test Evidence

Status: completed  
Evidence mode: Ran

## Executed Commands

1. Hourly-lane probe (`wepp_ui.txt` present):
   - `cargo run -q -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/hphys0234_20260601T215019Z/parity/runs --run-file /tmp/hphys0234_20260601T215019Z/parity/runs/p1_openwepp.run --output-dir /tmp/hphys0235_probe/out_hourly`
2. Daily-lane probe (`wepp_ui.txt` removed from copied run dir):
   - `cargo run -q -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/hphys0235_probe/runs_daily --run-file /tmp/hphys0235_probe/runs_daily/p1_openwepp.run --output-dir /tmp/hphys0235_probe/out_daily`
3. Lane provenance checks:
   - `jq '.mode_selection,.timestep_policy,.execution_provenance.selected_lane' /tmp/hphys0235_probe/out_hourly/openwepp_hillslope_run_manifest.json`
   - `jq '.mode_selection,.timestep_policy,.execution_provenance.selected_lane' /tmp/hphys0235_probe/out_daily/openwepp_hillslope_run_manifest.json`
4. Numeric comparison queries:
   - `duckdb` joins across hourly/daily/baseline `H1.wat.parquet`.

## Key Runtime Results

### Lane selection

- Hourly probe manifest: `effective=1`, `selected_lane="hourly"`,
  `timestep_seconds=3600`.
- Daily probe manifest: `effective=0`, `selected_lane="daily"`,
  `timestep_seconds=86400`.

### `H1` day-1..7 `Dp` ratios vs baseline

- Hourly lane average ratio: `7.259919373679516`
- Daily lane average ratio: `0.9416949711325998`

### Day-1..7 excerpt (`Dp`, mm/day)

| day | hourly | daily | baseline |
| ---: | ---: | ---: | ---: |
| 1 | 1.6468945549 | 0.0341225255 | 0.24 |
| 2 | 1.6848039854 | 0.1344139535 | 0.24 |
| 3 | 1.7191086735 | 0.2125027649 | 0.24 |
| 4 | 1.7499374379 | 0.2463939557 | 0.24 |
| 5 | 1.7767574826 | 0.2911331327 | 0.24 |
| 6 | 1.7998149066 | 0.3210509438 | 0.24 |
| 7 | 1.8193475069 | 0.3424302753 | 0.24 |

## Conclusion

Observed `~7x` mismatch is lane-semantic: hourly path remains materially
different from baseline hourly iterative behavior.
