# Gate Results

Status: complete
Evidence mode: Ran

Ran:

- `cargo fmt --check`: initial formatting failure on the new HPHYS0293 test; rerun after `cargo fmt` passed.
- `cargo test --test hphys0293_winter_melt_timing_contract -- --nocapture`: pass, `4 passed`.
- `cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture`: pass, `3 passed`.
- `cargo test --test hphys0292_spring_snowmelt_infiltration_capacity_contract -- --nocapture`: pass, `4 passed`.
- `.venv/bin/python docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/artifacts/hphys0293_diagnostics.py --run-root /tmp/hphys0293_full_20260604T212429Z --trace-max-days 1800`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with non-fatal existing warnings for duplicate crates (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances (`ISC`, `Unicode-DFS-2016`).
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`: pass, `2 passed`.
- `wctl doc-lint --path docs/work-packages/README.md`: pass, `1 files validated, 0 errors, 0 warnings`.
