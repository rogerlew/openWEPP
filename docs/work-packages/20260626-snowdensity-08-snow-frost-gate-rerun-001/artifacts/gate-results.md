# Gate Results

Evidence class: Ran.

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/snowdensity08_gate_rerun.py tools/snowfreeze_observed/coe_bound_density_adjudication.py tools/snowfreeze_observed/non_snotel_rubric_baseline.py`: pass.
- `.venv/bin/python tools/snowfreeze_observed/snowdensity08_gate_rerun.py`: pass.
- `cargo fmt --check`: pass.
- `cargo test --test snowdensity08_gate_rerun -- --nocapture`: pass.
- `cargo test --test snowdensity07_runtime_opt_in -- --nocapture`: pass.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`: pass after confinement allowlist update.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass after confinement allowlist update and full rerun.
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`).
- `rg -n "qwet|frzftp" crates`: no matches; exit 1 expected.
- `git diff --check`: pass.
