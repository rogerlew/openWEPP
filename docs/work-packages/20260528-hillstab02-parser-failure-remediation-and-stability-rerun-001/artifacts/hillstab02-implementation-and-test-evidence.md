# hillstab02-implementation-and-test-evidence

Status: complete  
Evidence mode: Ran

## Production Code Changes
- `crates/openwepp-input-contract/src/parsers/soil.rs`
  - Added compatibility-aware policy-row tokenization for `9002/9003/9005`
    rows containing quoted whitespace-bearing `luse`/`stext`.
  - Kept strict mode behavior unchanged.
- `crates/openwepp-input-contract/src/parsers/management.rs`
  - Added compatibility-only sentinel allowance for `tilseq=0` when `nseq>0`.
  - Kept strict mode positive index-domain enforcement unchanged.

## Commands
```bash
cargo test --test infile_soil_parser_contract --test infile_management_parser_contract
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
python docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py \
  --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --output-json /home/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02-rerun-results.json \
  --scratch-root /tmp/hillstab02 \
  --jobs 8 \
  --timeout-seconds 180
```

## Results
- Targeted parser suites: pass.
- Required workspace gates: pass.
- Cohort rerun executed successfully and emitted:
  - `artifacts/hillstab02-rerun-results.json`
- Stability disposition remains HOLD (see delta/disposition artifacts).
