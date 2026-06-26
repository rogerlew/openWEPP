# Gate Results

Evidence class: Ran.

- `cargo test --test snowdensity07_runtime_opt_in -- --nocapture`: pass.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`: pass.
- `cargo test --test snowdensity05d_opt_in_coe_melt -- --nocapture`: pass.
- `cargo test --test snowdensity06_density_compaction -- --nocapture`: pass.
- `cargo test --test snowdensity06b_coe_bound_density_replay -- --nocapture`: pass.
- `cargo test -p openwepp-hillslope-orchestrator --lib r7g_r4g_snow_coupling_mutates_winter_column_snow_state -- --nocapture`: pass.
- `cargo test -p openwepp-hillslope-orchestrator --lib r7g_executor_commits_r4g_winter_column_snow_state_to_lane -- --nocapture`: pass.
- `cargo test -p openwepp-hillslope-orchestrator --lib r7b_constructor_type_size_layout_is_bounded -- --nocapture`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`).
- `rg -n "qwet|frzftp" crates`: no matches; exit 1 expected.
- `git diff --check`: pass.
- `wctl doc-lint --path docs/work-packages/20260626-snowdensity-07-runtime-opt-in-001`:
  reported `0 files validated, 0 errors, 0 warnings`; not counted as a
  meaningful Markdown gate for this repo path.
