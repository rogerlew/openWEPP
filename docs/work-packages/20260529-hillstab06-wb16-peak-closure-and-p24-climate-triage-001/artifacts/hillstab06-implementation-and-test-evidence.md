# hillstab06-implementation-and-test-evidence

Status: complete  
Evidence mode: Ran

## Production Code Changes
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - added `WB16_RUNOFF_NEAR_ZERO_THRESHOLD = 1.0e-8`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - aligned WB16 near-zero branch trigger with baseline-compatible threshold,
  - permitted positive near-zero intermediates (removed epsilon-only rejection
    on `vave`, `vstar`, `te_base`, `tstar`, `tc`, `qpstar`),
  - retained finite checks and floor canonicalization (`peakro_raw.max(floor)`),
  - relaxed zero-branch writeback lower bounds for `tstar/qpstar/vstar` to
    `0.0`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - removed ordering-only `tmax < tmin` hard-fail checks in climate-coupled
    helper paths (snow/frost runtime helpers) while preserving finite/range
    guards.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - removed ordering-only `tmax < tmin` hard-fails in WB11 seed and WB13
    publication surfaces; finite-value validation remains.
- Test updates:
  - `tests/integration/wb16_peak_runoff_kernel_contract.rs`
  - `tests/integration/cli03_runner_contract_derived_tests.rs`

## Commands
```bash
cargo test --test wb16_peak_runoff_kernel_contract wb16_contract_conformance_accepts_near_zero_positive_runoff_with_floor_canonicalization
cargo test --test cli03_runner_contract_derived_tests cli03_runtime_accepts_finite_daily_temperature_inversion_records
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
python3 docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py \
  --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --scratch-root /tmp/hillstab06 \
  --output-json /home/workdir/openWEPP/docs/work-packages/20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001/artifacts/hillstab06-rerun-results.json
```

## Results
- Targeted contract-derived tests: pass (`2/2`).
- Required workspace gates: pass (`fmt`, `clippy`, `test`, `deny`).
- Release hillslope binary build: pass.
- Cohort rerun completed:
  - `wb05b_1166`: `1166/1166` passed
  - `release_gate_watchlist`: `19/19` passed
  - output written to `artifacts/hillstab06-rerun-results.json`.
