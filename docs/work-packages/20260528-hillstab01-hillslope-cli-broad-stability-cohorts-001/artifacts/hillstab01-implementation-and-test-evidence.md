# hillstab01-implementation-and-test-evidence

Status: complete  
Evidence mode: Ran

## Implemented
- Added stability harness:
  - `artifacts/hillstab01_stability_cohort.py`
- Added structured execution output:
  - `artifacts/hillstab01-stability-results.json`
- Added summary report:
  - `artifacts/hillstab01-stability-report.md`

## Commands
```bash
python -m py_compile docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
python docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py \
  --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --output-json /home/workdir/openWEPP/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01-stability-results.json \
  --scratch-root /tmp/hillstab01 \
  --jobs 8 \
  --timeout-seconds 180
```
