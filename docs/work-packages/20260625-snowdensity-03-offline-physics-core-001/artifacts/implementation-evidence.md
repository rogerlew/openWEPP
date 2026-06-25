# Implementation Evidence

Static:

- Added `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs`.
- Added `physics-bulk` to `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`.
- Re-exported the offline API from `openwepp-runner` for integration tests.
- Added `tools/snowfreeze_observed/physics_bulk_snotel_profile.py`.
- Added `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`.
- Registered the integration test in root `Cargo.toml`.

## Offline Outputs

`openwepp-snowbench physics-bulk` writes these caller-scoped artifacts:

- `physics_bulk_snow.csv`
- `physics_bulk_summary.json`
- `physics_bulk_summary.md`

The SNOTEL profile harness runs the CLI for all five SNOTEL fixtures, then emits:

- `physics_bulk_snotel_profile.json`
- `physics_bulk_snotel_profile.md`

Committed evidence copies:

- `artifacts/physics-bulk-snotel-profile.json`
- `artifacts/physics-bulk-snotel-profile.md`

## Boundary Evidence

The integration guard scans `crates/`, `tests/integration/`, and
`tools/snowfreeze_observed/` for `physics_bulk`. The allowed production symbols
are limited to snowbench module exports and the snowbench CLI command. The guard
does not allow production runtime activation paths, parser/config coupling,
publication schema changes, or default behavior changes.

## SNOTEL Profile Summary

Ran:

```bash
.venv/bin/python tools/snowfreeze_observed/physics_bulk_snotel_profile.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity03_physics_bulk_rerun \
  --snowbench-binary target/debug/openwepp-snowbench
```

Result:

- Schema: `snowdensity03-physics-bulk-snotel-profile-v1`.
- Site count: `5`.
- Paired rows: `13590`.
- All-cell counts: `fail=38`, `marginal=19`, `pass=3`, `strong=5`,
  `unavailable=15`.
- Forcing-robust counts: `fail=24`, `marginal=13`, `pass=3`, `strong=5`,
  `unavailable=15`.
- `openwepp_defective_cells = 0`.

Disposition: first candidate runs and emits finite rubric evidence, but is not
production-promotable. SNOWDENSITY-04 must adjudicate whether in-envelope
offline changes improve forcing-robust cells without per-site constants.
