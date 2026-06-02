# HPHYS0243 Run Evidence

Status: complete
Evidence mode: Ran

## Run Root

- `/tmp/hphys0243_20260602T042747Z/parity`

## Ran

- Verified required local roots:
  - `/tmp/unpalatable_parity_20260529T192707Z/runs`
  - `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions`
  - `/wc1/runs/un/unpalatable-rind/wepp/output/interchange`
  - `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
  - `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Built current runner binaries:
  - `cargo build -p openwepp-runner --bin openwepp-cli-hill --bin openwepp-cli-watershed`
- Executed fresh hillslope batch:
  - `target/debug/openwepp-cli-hill --run-dir /tmp/hphys0243_20260602T042747Z/parity/runs --run-file p{i}_openwepp.run --output-dir /tmp/hphys0243_20260602T042747Z/parity/hillslope_output --policy compat`
  - Result: `39/39` hillslopes completed with `rc=0`.
- Executed fresh watershed run:
  - `target/debug/openwepp-cli-watershed --run-dir /tmp/hphys0243_20260602T042747Z/parity/runs --run-file pw0_openwepp.run --output-dir /tmp/hphys0243_20260602T042747Z/parity/watershed_output --policy compat`
  - Result: `pw0 rc=0`.

## Status Files

- `/tmp/hphys0243_20260602T042747Z/parity/reports/hillslope_batch_status.tsv`
- `/tmp/hphys0243_20260602T042747Z/parity/reports/watershed_status.tsv`
- `/tmp/hphys0243_20260602T042747Z/parity/logs/`
