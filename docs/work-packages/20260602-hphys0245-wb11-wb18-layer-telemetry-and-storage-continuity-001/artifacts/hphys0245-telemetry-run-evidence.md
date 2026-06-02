# HPHYS0245 Telemetry Run Evidence

Status: completed
Evidence mode: Ran

## Run Root
- `/tmp/hphys0245_20260602T051933Z`

## Commands
- Built runner binary:
  `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- Ran targeted hillslopes using copied HPHYS0243 parity runfiles:
  - `OPENWEPP_HPHYS0245_TRACE_PATH=/tmp/hphys0245_20260602T051933Z/hillslope_output/H1.hphys0245.trace.jsonl`
  - `OPENWEPP_HPHYS0245_TRACE_PATH=/tmp/hphys0245_20260602T051933Z/hillslope_output/H7.hphys0245.trace.jsonl`
  - `OPENWEPP_HPHYS0245_TRACE_PATH=/tmp/hphys0245_20260602T051933Z/hillslope_output/H39.hphys0245.trace.jsonl`
  - `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=30`
  - `target/debug/openwepp-cli-hill --run-dir /tmp/hphys0245_20260602T051933Z/runs --run-file p<N>_openwepp.run --output-dir /tmp/hphys0245_20260602T051933Z/hillslope_output --policy compat`

## Results
- `H1`: return code `0`; trace rows `480`.
- `H7`: return code `0`; trace rows `480`.
- `H39`: return code `0`; trace rows `480`.
- Default-disabled smoke run: return code `0`; no HPHYS0245 sidecar found.

## Evidence Files
- `/tmp/hphys0245_20260602T051933Z/reports/telemetry_status.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/default_disabled_status.txt`
- `/tmp/hphys0245_20260602T051933Z/reports/default_disabled_files.txt`
- `/tmp/hphys0245_20260602T051933Z/hillslope_output/H1.hphys0245.trace.jsonl`
- `/tmp/hphys0245_20260602T051933Z/hillslope_output/H7.hphys0245.trace.jsonl`
- `/tmp/hphys0245_20260602T051933Z/hillslope_output/H39.hphys0245.trace.jsonl`
