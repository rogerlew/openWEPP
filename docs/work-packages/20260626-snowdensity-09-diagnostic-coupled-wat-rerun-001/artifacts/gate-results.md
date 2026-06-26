# SNOWDENSITY-09 Gate Results

Evidence class: Ran.

## Package Execution

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.
- `.venv/bin/python tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py`: pass.
- `.venv/bin/python tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py`: pass after v89 paired-snow gate correction; regenerated package JSON/Markdown artifacts with Mandan ND and Reynolds Creek ID reported as diagnostic-only out-of-gate sites.
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py tools/snowfreeze_observed/non_snotel_rubric_baseline.py`: pass.

## Focused Rust Gates

- `cargo fmt --check`: pass.
- `cargo test --test snowdensity09_coupled_wat_rerun -- --nocapture`: pass.
  - After the v89 gate correction, the first rerun failed on an over-specific
    contract marker string (`gate pass` vs the contract text `pass`). The marker
    was corrected and the focused test passed.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`: pass.
- `cargo test --test snowdensity07_runtime_opt_in -- --nocapture`: pass after updating the stale default-disabled guard to recognize the package-bound SNOWDENSITY-09 diagnostic env selector while still forbidding parser/runfile/user CLI activation.
- `cargo test --test snowdensity08_gate_rerun -- --nocapture`: pass after updating the stale default-path guard to preserve legacy default behavior and forbid user CLI activation rather than forbidding the new package-bound selector.
  - After the v89 gate correction, `cargo test --test snowdensity08_gate_rerun -- --nocapture` remained pass.

## Workspace Gates

These workspace gates are from the original SNOWDENSITY-09 execution before the
v89 paired-snow gate correction. The v89 correction is report/tooling/test
semantics only; focused v89 reruns are listed above and below.

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
  - Default all-site statuses: `SNOW_CONTROL_FAILED` for 3 paired-snow sites, `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` for 2 sites.
  - Default gate statuses: `SNOW_CONTROL_FAILED` for 3 gate-eligible paired-snow sites.
  - Opt-in all-site statuses: same classification.
  - Opt-in gate statuses: `SNOW_CONTROL_FAILED` for 3 gate-eligible paired-snow sites.
  - Out-of-gate diagnostic sites: `site3_scan_mandan_nd`, `site5_reynolds_creek_us_rls_id`.
- Final disposition: `COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`.
- Active blocker: `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`.

## Final Tree Checks

- `cargo fmt --check`: pass after gate artifact creation.
- `git diff --check`: pass after gate artifact creation.
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py tools/snowfreeze_observed/non_snotel_rubric_baseline.py`: pass after v89 gate correction.
- `cargo fmt --check`: pass after v89 gate correction.
- `git diff --check`: pass after v89 gate correction.
