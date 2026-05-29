# hillstab03-implementation-and-test-evidence

Status: complete  
Evidence mode: Ran

## Production Code Changes
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - removed WB16 runtime dependency on `timep`,
  - changed WB16 exponent domain guard from `m > 1` to `m > 0`,
  - removed WB16 upper-bound reject for `vstar > 1`,
  - implemented baseline `tc(vstar)` branch partition for `vstar < 1`,
  - added explicit constant-excess branch (`method_branch=4`, `qpstar=1`) for
    `vstar >= 1` with `tstar < 1`,
  - extended WB16 branch publication bounds from `1..3` to `1..4`.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - removed now-unused WB16 `timep` symbol constant.

## Commands
```bash
cargo test --test wb16_peak_runoff_kernel_contract
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
python docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py \
  --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --output-json /home/workdir/openWEPP/docs/work-packages/20260528-hillstab03-wb16-peak-domain-closure-001/artifacts/hillstab03-rerun-results.json \
  --scratch-root /tmp/hillstab03 \
  --jobs 8 \
  --timeout-seconds 180
```

## Results
- Targeted WB16 contract suite: pass (`5/5`).
- Required workspace gates: pass (`fmt`, `clippy`, `test`, `deny`).
- Release hillslope binary build: pass.
- Cohort rerun completed and emitted:
  - `artifacts/hillstab03-rerun-results.json`.
