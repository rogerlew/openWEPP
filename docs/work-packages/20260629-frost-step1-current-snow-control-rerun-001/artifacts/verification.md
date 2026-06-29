# Verification

Evidence mode: Ran.

## Commands

| Gate | Command | Result |
| --- | --- | --- |
| CLI build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | PASS |
| Observation corpus | `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate` | PASS |
| Five-site harness | `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare ...` for all five sites | PASS |
| Legacy scalar audit | `.venv/bin/python tools/snowfreeze_observed/snow_depth_audit.py ...` | PASS |
| Legacy classifier | `.venv/bin/python tools/snowfreeze_observed/classify_residuals.py ...` | PASS |
| Step 1 routing | `.venv/bin/python artifacts/route_current_snow_control.py ...` | PASS |

## Gate Results

- Five comparison reports exist in `artifacts/site_reports/`: PASS.
- Per-site routing table exists in `current_snow_control_routing.md/json`: PASS.
- Paired snow sites are separated into forcing-limited versus blocked:
  PASS (`site1`, `site2` forcing-limited; `site4` blocked).
- Sites without paired snow depth are inconclusive, not treated as pass:
  PASS (`site3`, `site5`).
- No snow/frost runtime, fixture, schema, default, or contract physics file was
  edited: PASS.

## Residual Risk

The Sleepers sites are not scalar snow-depth passes. Step 2 must carry
absolute snow-depth magnitude uncertainty explicitly and must avoid converting
forcing-limited magnitude residuals into frost-model defects.
