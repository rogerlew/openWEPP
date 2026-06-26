# SNOWDENSITY-09 Gate Results

Evidence class: Ran.

## Package Execution

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.
- `.venv/bin/python tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py`: pass.
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py tools/snowfreeze_observed/non_snotel_rubric_baseline.py`: pass.

## Focused Rust Gates

- `cargo fmt --check`: pass.
- `cargo test --test snowdensity09_coupled_wat_rerun -- --nocapture`: pass.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`: pass.
- `cargo test --test snowdensity07_runtime_opt_in -- --nocapture`: pass after updating the stale default-disabled guard to recognize the package-bound SNOWDENSITY-09 diagnostic env selector while still forbidding parser/runfile/user CLI activation.
- `cargo test --test snowdensity08_gate_rerun -- --nocapture`: pass after updating the stale default-path guard to preserve legacy default behavior and forbid user CLI activation rather than forbidding the new package-bound selector.

## Workspace Gates

- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
  - First full-workspace attempt failed in stale SNOWDENSITY-07 and SNOWDENSITY-08 guard assertions that predated the SNOWDENSITY-09 contract authority. Those guards were revised to keep the intended invariant: default path remains legacy and no parser/runfile/user CLI activation exists.
  - Final rerun passed.
- `cargo deny check`: pass.

## Anti-Evasion / Source Scans

- `rg -n "qwet|frzftp" crates`: no matches. Command returned exit code 1 as expected for no matches.
- SNOWDENSITY-09 integration guard verifies `openwepp-cli-hill` does not expose `physics_bulk_density_compaction_v1` as a user CLI selector.
- SNOWDENSITY-09 integration guard verifies the diagnostic selector is confined to `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL` and that unknown/non-UTF-8 values fail closed.

## Disposition Evidence

- Coupled opt-in WAT path executed for all five frost-observation sites.
- Default direct snow trace rows: `75610`, all `legacy_wepp`.
- Opt-in direct snow trace rows: `75610`, all `physics_bulk_density_compaction_v1`.
- SNOTEL density gate from SNOWDENSITY-08 remained cleared.
- Non-SNOTEL snow-control gate did not clear:
  - Default statuses: `SNOW_CONTROL_FAILED` for 3 paired-snow sites, `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` for 2 sites.
  - Opt-in statuses: same classification.
- Final disposition: `COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`.
- Active blocker: `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`.

## Final Tree Checks

- `cargo fmt --check`: pass after gate artifact creation.
- `git diff --check`: pass after gate artifact creation.
