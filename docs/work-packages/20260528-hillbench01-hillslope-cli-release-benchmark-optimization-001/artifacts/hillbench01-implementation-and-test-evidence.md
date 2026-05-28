# HILLBENCH01 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Implemented scoped hillslope CLI/runtime performance edits:
  - `crates/openwepp-runner/src/release.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Added benchmark harness and persisted benchmark captures:
  - `artifacts/hillbench01_release_benchmark.py`
  - `artifacts/hillbench01-pre-optimization-benchmark.json`
  - `artifacts/hillbench01-post-optimization-benchmark.json`

## Ran
1. Release build:
   - `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` -> pass
2. Benchmark harness (pre-optimization):
   - `python3 .../artifacts/hillbench01_release_benchmark.py --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --repetitions 12 --warmups 2 --output-json /tmp/hillbench01/results/pre_optimization.json` -> pass
3. Benchmark harness (post-optimization):
   - `python3 .../artifacts/hillbench01_release_benchmark.py --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --repetitions 12 --warmups 2 --output-json /tmp/hillbench01/results/post_optimization.json` -> pass
4. Required validation gates:
   - `cargo fmt --check` -> pass
   - `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - `cargo test --workspace` -> pass
   - `cargo deny check` -> pass (warnings-only duplicates/unmatched allow-list entries; final status `advisories ok, bans ok, licenses ok, sources ok`)
